# BDK Dojo Context Discipline

## Problem

Large reference files can make agents lazy or noisy. BDK Dojo should load only what the current lesson needs.

## Minimal load order

For any BDK Dojo lesson, load:

1. `SKILL.md`
2. `references/context-discipline.md`
3. `references/teach-mode.md`
4. `references/teacher-style.md`
5. `references/answer-validation.md`
6. `references/course-spine.md`
7. the current `scaffold/<lesson>/README.md`
8. the current `scaffold/<lesson>/stubs.rs`

Load other references only when needed:

- `bitcoin-dojo-format.md` — when changing repo/scaffold layout
- `bdk-learning-coverage.md` — when auditing curriculum coverage
- `progress-journal.md` — when writing or updating progress notes
- `teacher-style.md` — already part of the minimal load; reload if the response starts sounding scripted
- `wallet-balance-utxo-model.md` — legacy/reference balance lesson only

The current lesson source of truth is the numbered `scaffold/<lesson>/README.md` plus `stubs.rs`, not older reference lessons.

## Agent rule

Do not load every reference file by default.

Use the course spine to pick the lesson. Then confirm `scaffold/<lesson>/README.md` and `scaffold/<lesson>/stubs.rs` exist. Those files are the readiness signal, matching `SaucePackets/bitcoin-dojo`.

If the next course-spine lesson is planned but its scaffold directory is missing, author that single next scaffold before teaching it.

## Lesson handoff shape

For first lesson openings, use the fuller coach style from `teach-mode.md`: lane, pattern, language, difficulty, goal, concrete problem, file tree, starter shape, expected cursed behavior, one job, and exact send-back request.

For follow-ups and reviews, keep it compact:

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
