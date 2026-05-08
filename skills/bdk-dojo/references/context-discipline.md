# BDK Dojo Context Discipline

## Problem

Large reference files can make agents lazy or noisy. BDK Dojo should load only what the current lesson needs.

## Minimal load order

For any BDK Dojo lesson, load:

1. `SKILL.md`
2. `references/context-discipline.md`
3. `references/teach-mode.md`
4. `references/course-spine.md`
5. the current `scaffold/<lesson>/README.md`
6. the current `scaffold/<lesson>/stubs.rs`

Load other references only when needed:

- `bitcoin-dojo-format.md` — when changing repo/scaffold layout
- `bdk-learning-coverage.md` — when auditing curriculum coverage
- `progress-journal.md` — when writing or updating progress notes
- `wallet-balance-utxo-model.md` — legacy/reference balance lesson only

The current lesson source of truth is the numbered `scaffold/<lesson>/README.md` plus `stubs.rs`, not older reference lessons.

## Agent rule

Do not load every reference file by default.

Use the course spine to pick the lesson. Then read the specific scaffold files for that lesson.

## Lesson handoff shape

When responding, keep the lesson compact:

```text
Lesson:
Builds on:
Files:
Concept:
Rust focus:
Task:
Tests:
Hint:
Stop after:
```

## If context feels overloaded

Summarize the already-read reference in 5 bullets, then continue from the scaffold.
