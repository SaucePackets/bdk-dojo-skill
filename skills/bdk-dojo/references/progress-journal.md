# BDK Dojo Progress Journal

## Purpose

BDK Dojo should track learning progress, not just code progress.

This journal is for the learner's private practice repo, not the public curriculum repo. It records completed lessons, what changed in the wallet model, what the learner understood, and what pain points they overcame.

Do not put private coaching notes, embarrassment logs, or sensitive personal details in the public `bdk-dojo-skill` repo.

## Recommended learner repo files

```text
bdk-dojo/
  notes/
    progress.md
    pain-points.md
    questions.md
  lessons/
    completed.md
```

## Lesson completion entry

Append one entry after each completed lesson:

```markdown
## 1.2 — Total Balance

Date:
Commit:
Status: complete

Built:
- `src/balance.rs`
- `calculate_balance(utxos: &[Utxo]) -> u64`

Tests passed:
- `empty_wallet_returns_zero`
- `sums_multiple_utxos`

Learned:
- Wallet balance starts from UTXOs, not an account row.
- Sats should stay integers.

Pain points overcome:
- Remembered package names with hyphens import as underscores.
- Fixed borrow/slice confusion around `&[Utxo]`.

Still fuzzy:
- Difference between confirmed balance and spendable balance.

Next:
- 1.3 — Balance Buckets
```

## Agent behavior

At the end of every lesson, the teaching agent should ask for or draft a short progress entry with:

- lesson number and title
- files created or changed
- functions/types built
- tests that passed
- concept learned
- Rust pain point, if any
- Bitcoin/BDK pain point, if any
- what improved since the prior attempt
- active review targets
- next lesson

Keep it factual and useful. No therapy diary. No fluff.

## Privacy rule

Public repo:

- curriculum
- scaffold instructions
- starter stubs
- generic lesson notes

Private learner repo:

- completed lessons
- personal pain points
- coaching feedback
- questions
- progress journal

Persistent agent memory:

- only durable preferences or major recurring learning patterns
- not every lesson result
- not temporary TODOs

## Completion criteria

A lesson is complete only when:

- implementation compiles
- required tests pass
- at least one edge case exists
- the learner can explain the Bitcoin/BDK concept in one or two sentences
- any stated uncertainty has been explained and reinforced before completion
- active pain points are recorded as review targets, not treated as failure
- the progress journal has a short entry
