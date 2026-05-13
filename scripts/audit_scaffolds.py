#!/usr/bin/env python3
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
SCAFFOLD = ROOT / "scaffold"
COURSE_SPINE = ROOT / "skills" / "bdk-dojo" / "references" / "course-spine.md"
errors: list[str] = []

if not SCAFFOLD.exists():
    errors.append("missing scaffold/ directory")
else:
    scaffold_dirs = sorted(p for p in SCAFFOLD.iterdir() if p.is_dir())
    if not scaffold_dirs:
        errors.append("scaffold/ has no lesson directories")

    lesson_ids = set()
    for lesson in scaffold_dirs:
        rel = lesson.relative_to(ROOT)
        lesson_ids.add(lesson.name.split("-", 1)[0])
        readme = lesson / "README.md"
        stubs = lesson / "stubs.rs"
        if not readme.exists():
            errors.append(f"{rel}: missing README.md")
            continue
        if not stubs.exists():
            errors.append(f"{rel}: missing stubs.rs")
        text = readme.read_text(encoding="utf-8")
        checks = {
            "## Setup": "## Setup" in text,
            "## Goal": "## Goal" in text,
            "## Expected behavior": "## Expected behavior" in text,
            "required proof/tests/artifact": any(h in text for h in ["## Required tests", "## Required proof", "## Required artifact"]),
            "## Reference implementation": "## Reference implementation" in text,
            "## BDK bridge": "## BDK bridge" in text,
            "## Done when": "## Done when" in text,
        }
        for label, ok in checks.items():
            if not ok:
                errors.append(f"{rel}: missing {label}")
        if "Only compare to the reference after the learner attempts" not in text:
            errors.append(f"{rel}: missing learner-first reference warning")

    if COURSE_SPINE.exists():
        spine = COURSE_SPINE.read_text(encoding="utf-8")
        spine_ids = set(re.findall(r"\b([1-5]\.\d)\b", spine))
        missing = sorted(spine_ids - lesson_ids)
        extra = sorted(lesson_ids - spine_ids)
        if missing:
            errors.append("course spine lessons missing scaffolds: " + ", ".join(missing))
        if extra:
            errors.append("scaffold lessons missing from course spine: " + ", ".join(extra))
    else:
        errors.append("missing course spine reference")

if errors:
    print("Scaffold audit failed:")
    for err in errors:
        print(f"- {err}")
    sys.exit(1)

print(f"OK: audited {len(scaffold_dirs)} scaffold lessons")
