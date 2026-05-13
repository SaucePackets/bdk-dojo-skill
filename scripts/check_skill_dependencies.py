#!/usr/bin/env python3
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
errors: list[str] = []

vendored = ROOT / "skills" / "teach-mode" / "SKILL.md"
if vendored.exists():
    errors.append("teach-mode is vendored; install it as standalone dependency instead")

bdk_skill = ROOT / "skills" / "bdk-dojo" / "SKILL.md"
text = bdk_skill.read_text(encoding="utf-8")
if "related_skills: [teach-mode" not in text:
    errors.append("bdk-dojo SKILL.md must list teach-mode in related_skills")
if "external dependency" not in text.lower() and "standalone umbrella `teach-mode`" not in text:
    errors.append("bdk-dojo SKILL.md must state teach-mode is standalone/external")

for rel in ["README.md", "docs/install-hermes.md"]:
    body = (ROOT / rel).read_text(encoding="utf-8")
    if "SaucePackets/teach-mode-skill/skills/teach-mode" not in body:
        errors.append(f"{rel} must include standalone teach-mode install path")
    if "skills/teach-mode/SKILL.md" in body:
        errors.append(f"{rel} still documents a vendored teach-mode SKILL.md")

installer = ROOT / "scripts" / "install-hermes.sh"
if not installer.exists():
    errors.append("missing scripts/install-hermes.sh")
else:
    script = installer.read_text(encoding="utf-8")
    required = [
        'TEACH_MODE_REF="${TEACH_MODE_REF:-v0.0.1}"',
        'BDK_DOJO_REF="${BDK_DOJO_REF:-v0.0.2}"',
        "SaucePackets/teach-mode-skill",
        "SaucePackets/bdk-dojo-skill",
        "skills install",
    ]
    for phrase in required:
        if phrase not in script:
            errors.append(f"scripts/install-hermes.sh missing {phrase}")

if errors:
    print("Skill dependency validation failed:")
    for err in errors:
        print(f"- {err}")
    sys.exit(1)

print("OK: bdk-dojo declares teach-mode as a standalone dependency")
