#!/usr/bin/env python3
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
SCAFFOLD = ROOT / "scaffold"
COURSE_SPINE = ROOT / "skills" / "bdk-dojo" / "references" / "course-spine.md"
errors: list[str] = []
LESSON_ID_RE = re.compile(r"^(\d+\.\d+)-")
SPINE_ID_RE = re.compile(r"\b(\d+\.\d+)\b")


def section_body(text: str, heading: str) -> str:
    pattern = re.compile(rf"^## {re.escape(heading)}\s*$", re.M)
    match = pattern.search(text)
    if not match:
        return ""
    rest = text[match.end():]
    next_heading = re.search(r"^## ", rest, re.M)
    return rest[: next_heading.start()] if next_heading else rest


def reference_paths(text: str) -> list[str]:
    body = section_body(text, "Reference implementation")
    paths: list[str] = []
    for line in body.splitlines():
        for item in re.findall(r"`([^`]+)`", line):
            if item.endswith((".rs", ".md")) or item.startswith(("examples/", "scaffold/", "skills/")):
                paths.append(item)
    return paths


if not SCAFFOLD.exists():
    errors.append("missing scaffold/ directory")
    scaffold_dirs: list[Path] = []
else:
    scaffold_dirs = sorted(p for p in SCAFFOLD.iterdir() if p.is_dir())
    if not scaffold_dirs:
        errors.append("scaffold/ has no lesson directories")

lesson_ids = set()
for lesson in scaffold_dirs:
    rel = lesson.relative_to(ROOT)
    m = LESSON_ID_RE.match(lesson.name)
    if not m:
        errors.append(f"{rel}: directory name must start with lesson id like 5.10-")
        continue
    lesson_ids.add(m.group(1))

    readme = lesson / "README.md"
    stubs = lesson / "stubs.rs"
    lesson_test = lesson / f"lesson_{lesson.name.replace('-', '_').replace('.', '_')}.rs"
    legacy_test = lesson / "tests.rs"
    if not readme.exists():
        errors.append(f"{rel}: missing README.md")
        continue
    if not stubs.exists():
        errors.append(f"{rel}: missing stubs.rs")
    elif not stubs.read_text(encoding="utf-8").strip():
        errors.append(f"{rel}: stubs.rs is empty")
    if legacy_test.exists():
        errors.append(f"{rel}: legacy tests.rs found; use {lesson_test.name}")
    if lesson_test.exists():
        lesson_test_text = lesson_test.read_text(encoding="utf-8")
        if "your_crate_name" in lesson_test_text:
            errors.append(f"{rel}: lesson test must use fixed crate import `bdk_dojo`, not your_crate_name")

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

    for ref in reference_paths(text):
        if not (ROOT / ref).exists():
            errors.append(f"{rel}: reference implementation path does not exist: {ref}")

    is_reflection = any(
        phrase in text.lower()
        for phrase in [
            "markdown artifact",
            "contribution note",
            "capstone explanation",
            "safe pseudocode",
            "checklist/reference helper",
        ]
    )
    if is_reflection and "## Required tests" in text:
        errors.append(f"{rel}: reflection/Markdown lesson must use ## Required artifact or ## Required proof, not ## Required tests")
    if "## Required tests" in text:
        if not lesson_test.exists():
            errors.append(f"{rel}: missing lesson acceptance test file {lesson_test.name}")
        if lesson_test.name not in text:
            errors.append(f"{rel}: README must name lesson acceptance test file {lesson_test.name}")
        if "match your Cargo.toml package name" in text:
            errors.append(f"{rel}: README should not make learners choose a crate name; use package `bdk-dojo` / import `bdk_dojo`")

    mentions_real_bdk = any(
        phrase in text
        for phrase in [
            "bdk_wallet::",
            "bdk_chain::",
            "bitcoindevkit/bdk",
            "bdk_file_store",
            "ChangeSet",
            "Esplora",
            "Electrum",
            "bitcoind RPC",
            "CONTRIBUTING.md",
        ]
    )
    if mentions_real_bdk and "## BDK source stamp" not in text:
        errors.append(f"{rel}: real BDK API/source mentions need ## BDK source stamp")

if COURSE_SPINE.exists():
    spine = COURSE_SPINE.read_text(encoding="utf-8")
    spine_ids = set(SPINE_ID_RE.findall(spine))
    missing = sorted(spine_ids - lesson_ids, key=lambda x: [int(p) for p in x.split(".")])
    extra = sorted(lesson_ids - spine_ids, key=lambda x: [int(p) for p in x.split(".")])
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
