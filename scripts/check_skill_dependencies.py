#!/usr/bin/env python3
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
EXPECTED_VERSION = "0.0.4"
errors: list[str] = []


def parse_frontmatter(path: Path) -> tuple[dict[str, str], str]:
    text = path.read_text(encoding="utf-8")
    if not text.startswith("---\n"):
        errors.append(f"{path.relative_to(ROOT)} must start with YAML frontmatter")
        return {}, text
    end = text.find("\n---\n", 4)
    if end == -1:
        errors.append(f"{path.relative_to(ROOT)} frontmatter is not closed")
        return {}, text
    fm_text = text[4:end]
    fields: dict[str, str] = {}
    lines = fm_text.splitlines()
    for i, line in enumerate(lines):
        stripped = line.strip()
        if not stripped or stripped.startswith("#"):
            continue
        if not line.startswith(" ") and ":" in line:
            key, value = line.split(":", 1)
            fields[key.strip()] = value.strip().strip('"')
    related = re.search(r"related_skills:\s*\[([^\]]*)\]", fm_text)
    if related:
        fields["related_skills"] = related.group(1)
    return fields, text


vendored_dir = ROOT / "skills" / "teach-mode"
if vendored_dir.exists():
    errors.append("skills/teach-mode/ must not exist; install Teach Mode as standalone dependency")

bdk_skill = ROOT / "skills" / "bdk-dojo" / "SKILL.md"
frontmatter, text = parse_frontmatter(bdk_skill)
if frontmatter.get("version") != EXPECTED_VERSION:
    errors.append(f"bdk-dojo SKILL.md version must be {EXPECTED_VERSION}")
related = frontmatter.get("related_skills", "")
if not re.search(r"\bteach-mode\b", related):
    errors.append("bdk-dojo SKILL.md must list teach-mode in metadata.hermes.related_skills")
if "## Required Companion Skill" not in text:
    errors.append("bdk-dojo SKILL.md must include ## Required Companion Skill")
if "SaucePackets/teach-mode-skill/skills/teach-mode" not in text:
    errors.append("bdk-dojo SKILL.md must name standalone Teach Mode install path")
if "does not vendor Teach Mode" not in text:
    errors.append("bdk-dojo SKILL.md must state it does not vendor Teach Mode")

for rel in ["README.md", "docs/install-hermes.md", "docs/install-openclaw.md"]:
    body = (ROOT / rel).read_text(encoding="utf-8")
    if "SaucePackets/teach-mode-skill/skills/teach-mode" not in body:
        errors.append(f"{rel} must mention standalone Teach Mode dependency")
    forbidden_vendor_markers = [
        "bdk-dojo-skill/skills/teach-mode/SKILL.md",
        "`skills/teach-mode/SKILL.md`",
        "includes `teach-mode` directly",
    ]
    for marker in forbidden_vendor_markers:
        if marker in body:
            errors.append(f"{rel} still documents vendored teach-mode marker: {marker}")

installer = ROOT / "scripts" / "install-hermes.sh"
if not installer.exists():
    errors.append("missing scripts/install-hermes.sh")
else:
    script = installer.read_text(encoding="utf-8")
    required = [
        'TEACH_MODE_ID="${TEACH_MODE_ID:-SaucePackets/teach-mode-skill/skills/teach-mode}"',
        'BDK_DOJO_ID="${BDK_DOJO_ID:-SaucePackets/bdk-dojo-skill/skills/bdk-dojo}"',
        '"$HERMES_BIN" skills install "$identifier" --category "$CATEGORY" --yes --force',
    ]
    for phrase in required:
        if phrase not in script:
            errors.append(f"scripts/install-hermes.sh missing {phrase}")

    if script.count("--force") != 1:
        errors.append("scripts/install-hermes.sh must use --force only in the shared Hermes install command")
    forbidden_complexity = ["skill_visible()", "already installed"]
    for phrase in forbidden_complexity:
        if phrase in script:
            errors.append(f"scripts/install-hermes.sh should just install both skills; remove extra skip logic: {phrase}")
    for phrase in ["install_skill()", "Done. Start a new Hermes session", "Use bdk-dojo. Give me the first wallet-balance kata."]:
        if phrase not in script:
            errors.append(f"scripts/install-hermes.sh missing expected simple installer behavior: {phrase}")

if errors:
    print("Skill dependency validation failed:")
    for err in errors:
        print(f"- {err}")
    sys.exit(1)

print(f"OK: bdk-dojo declares standalone Teach Mode dependency and version {EXPECTED_VERSION}")
