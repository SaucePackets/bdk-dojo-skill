#!/usr/bin/env python3
from pathlib import Path
import os
import sys
import urllib.request

ROOT = Path(__file__).resolve().parents[1]
VENDORED = ROOT / "skills" / "teach-mode" / "SKILL.md"
LOCAL_UPSTREAM = ROOT.parent / "teach-mode-skill" / "skills" / "teach-mode" / "SKILL.md"
REMOTE_UPSTREAM = "https://raw.githubusercontent.com/SaucePackets/teach-mode-skill/main/skills/teach-mode/SKILL.md"

vendored = VENDORED.read_text(encoding="utf-8")
source = ""
upstream = ""

if LOCAL_UPSTREAM.exists():
    source = str(LOCAL_UPSTREAM)
    upstream = LOCAL_UPSTREAM.read_text(encoding="utf-8")
else:
    source = REMOTE_UPSTREAM
    try:
        with urllib.request.urlopen(REMOTE_UPSTREAM, timeout=20) as response:
            upstream = response.read().decode("utf-8")
    except Exception as exc:
        print(f"Could not fetch upstream teach-mode from {REMOTE_UPSTREAM}: {exc}")
        sys.exit(1)

if vendored != upstream:
    print("Vendored teach-mode drift detected.")
    print(f"Vendored: {VENDORED.relative_to(ROOT)}")
    print(f"Upstream: {source}")
    print("Update bdk-dojo-skill/skills/teach-mode/SKILL.md from upstream before merging.")
    sys.exit(1)

print(f"OK: vendored teach-mode matches {source}")
