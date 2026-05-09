---
name: bdk-dojo
description: "Use when teaching or practicing Bitcoin wallet engineering through Rust/BDK-shaped mini-katas: UTXOs, descriptors, PSBTs, wallet sync, fee estimation, tests, and open-source contribution readiness."
version: 1.0.0
author: Hermes Agent
license: MIT
metadata:
  hermes:
    tags: [bitcoin, rust, bdk, education, dojo, wallet, open-source]
    related_skills: [test-driven-development, systematic-debugging]
---

# BDK Dojo

## Overview

BDK Dojo is a learning-first workflow for Bitcoin wallet engineering practice.

It uses small Rust exercises to teach the concepts behind wallet libraries like BDK: UTXOs, descriptors, PSBTs, wallet sync, fee estimation, ownership, error handling, and test discipline.

The goal is not to pretend the learner is ready to patch BDK on day one. The goal is to build the habits that make future contribution credible: small changes, readable code, tests, clear questions, and accurate mental models.

## When to Use

Use this skill when the user asks for:

- a BDK Dojo lesson
- Bitcoin/Rust practice
- wallet engineering exercises
- BDK concept explanations
- UTXO, descriptor, PSBT, sync, or fee-estimation katas
- preparation for Bitcoin open-source contribution
- meetup-friendly Bitcoin developer lessons

Do not use this for:

- real wallet security decisions
- production key handling
- signing real transactions
- legal, tax, or investment advice
- replacing the official BDK documentation

## Core Stance

The learner writes first.

The agent may scaffold project shape, explain Rust file layout, give hints, review code, write tests after the attempt, and point toward official documentation. Do not write the full first solution unless the learner explicitly asks for rescue.

Keep lessons small. One concept. One kata. One verification loop.

Teach Mode is the default. Follow `references/teach-mode.md` and `references/teacher-style.md`: learner writes first, agent gives hints before rescue, verifies tests, explains fuzzy concepts before completion, asks for a short learner explanation, and records learning.

BDK Dojo is learning-first, not issue-claim-first. Explore BDK, read examples, build small local features, and understand contribution guidelines before attempting real upstream work.

Every lesson should strengthen four tracks at once:

- Rust comfort
- Bitcoin wallet mental model
- BDK concept/API mapping
- contribution-ready habits

## Default Lesson Flow

Before choosing a lesson, use the compact load order in `references/context-discipline.md`. Do not load every reference file by default.

Default lesson references:

- `references/context-discipline.md`
- `references/teach-mode.md`
- `references/teacher-style.md`
- `references/answer-validation.md`
- `references/course-spine.md`
- current scaffold README/stubs

Load larger references only when needed for layout audits, coverage audits, or progress-journal updates.

1. Pick the next uncompleted concept from the course spine.
2. Check the learner repo's progress notes and active pain points when available.
3. Inspect the current scaffold README/stubs and relevant learner files when available.
4. Open by connecting the new lesson to what the learner just learned.
5. Explain why it matters in wallet engineering.
6. Show what repo state it builds on.
7. Give a tiny Rust exercise with clear directions but not the core answer.
8. Make the learner implement the core logic first.
9. Require at least one normal test and one edge-case test.
10. Verify with plain `cargo test` and, when relevant, `cargo run`.
11. Compare against the expected behavior and, after the learner attempts, the reference crate when useful.
12. Review compiler success, warnings, correctness, and code clarity separately.
13. Ask for one improvement or refactor before showing polished code.
14. Tie the lesson back to real BDK docs, examples, APIs, or contribution habits.
15. Have the learner explain the concept in one or two sentences.
16. End by drafting or updating a short progress-journal entry: completed lesson, files changed, tests passed, concept learned, pain points overcome, active review targets, and next lesson.

Do not mark a lesson complete just because tests pass. If the learner says a concept is fuzzy, explain it, connect it to their code, ask them to restate it, then journal the pain point and progress after understanding improves.

## Lesson Handoff Format

Keep lesson starts energetic and concrete. Do not reduce the lesson to a checklist.

Use this shape for the first handoff:

```text
There you are. BDK Dojo starts.

Lane:
Pattern:
Language:
Difficulty:
Goal:

Problem:

[plain-English problem statement]

You will create/update:

[small file tree]

What each file means:

[short explanation]

Put this starter shape in:

[small scaffold excerpt with todo! where learner must work]

Run:

[cargo command]

Expected cursed behavior:

[compile error or todo panic if applicable]

Your job:

[one focused implementation task]

Hint:

[one useful hint]

Send me:

[specific file or error]
```

For compact follow-ups or reviews, use this as an internal checklist, not a script:

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

Beginner lessons should scaffold project shape before asking for implementation. Explain `Cargo.toml`, `src/lib.rs`, `src/main.rs`, module exports, and crate-name underscore traps when relevant. No tutorial coma, but enough orientation that a beginner knows the next keystroke.

## Review Format

```text
Verdict:
What worked:
What broke:
Why Rust is mad:
Smallest fix:
Rust concept learned:
Bitcoin model:
Tests:
BDK bridge:
Pain point:
Your move:
Next kata:
```

Review attempts like a coach. Start with the correct shape, diagnose one failure clearly, and give the smallest next action. Do not dump the polished reference unless the learner asks for rescue. The labels above are for consistency; the final response should sound natural, not like a grading form.

## Beginner Track

Start with Rust comfort and wallet primitives. Work from the Bitcoin Dojo-style scaffolds into the cumulative working crate at `examples/bdk-dojo-wallet/`.

Important distinction:

- `references/course-spine.md` is the full planned curriculum.
- `scaffold/<lesson>/README.md` plus `scaffold/<lesson>/stubs.rs` means that lesson is authored and teachable.
- There is no central `scaffold/README.md`; the scaffold directories themselves are the index, matching `SaucePackets/bitcoin-dojo`.
- Only teach lessons that have a scaffold README, stubs, expected behavior, required tests, and reference path.

Authored scaffolds accumulate as lessons are taught. Do not delete prior authored scaffolds just to make the repo look like only lesson 1 exists; they are part of the reusable curriculum history, like Bitcoin Dojo's completed scaffold directories.

Authored so far:

- `scaffold/1.1-amounts-and-utxos` -> `src/amount.rs`, `src/utxo.rs`.
- `scaffold/1.2-total-balance` -> `src/balance.rs`, `calculate_balance`.
- `scaffold/1.3-balance-buckets` -> `BalanceSummary`, `classify_balance`.
- `scaffold/1.4-wallet-state` -> `src/wallet.rs`, `WalletState`, `WalletState::balance`.

Then continue authoring from `references/course-spine.md`: confirmations, spendability, sync, fees, coin selection, transaction proposals, PSBT review, and BDK bridge lessons.

Default starting point is the next incomplete *authored scaffold*, not a planned-but-unauthored course-spine item.

Scaffold authoring should follow the Bitcoin Dojo pattern: author the next lesson one at a time as it becomes current, with `README.md` explicitly describing any file, type, function, or test changes required by the evolving cumulative crate. Do not pre-scaffold distant lessons just for consistency, because future lessons may depend on design changes discovered during earlier practice. Do not remove already-authored scaffolds after completion. Do not leave the learner without a next step either: when the next course-spine lesson is not scaffolded yet, author that single next scaffold before teaching it.

## Intermediate Track

Move from primitives into BDK-shaped implementation:

- descriptor strings as wallet policy descriptions
- address derivation mental model
- PSBT review flow
- wallet sync states
- fee-rate units and estimation
- transaction size and vbytes
- coin selection tradeoffs
- persistence and checkpointing concepts

Use toy models first, then point to official BDK examples.

## Answer Validation

Correctness should come from explicit artifacts, not agent intuition.

Use `references/answer-validation.md`:

- scaffold README defines expected behavior
- scaffold stubs define API shape
- tests prove normal and edge behavior
- `examples/bdk-dojo-wallet/` is the public reference implementation after the learner attempts
- BDK bridge notes verify the concept maps to real BDK accurately

The agent should not dump the reference implementation before the learner tries.

## BDK Bridge Discipline

Toy lessons are only useful if they eventually map to BDK.

When a toy concept stabilizes, add a short bridge note:

```text
Toy concept:
Closest BDK concept/API/example:
What the toy hides:
What to read next:
```

For real BDK exploration:

- inspect current BDK docs/examples before teaching API details
- prefer regtest or signet examples
- read contribution guidelines before upstream work
- build small local examples before claiming issues
- start with docs/tests/examples, not core rewrites

## Contribution Track

Only after the learner can write and test small Rust modules:

- read one small BDK issue or PR
- reproduce a docs example locally
- improve docs or tests first
- ask a focused maintainer question
- submit tiny, reviewable changes
- avoid drive-by rewrites

Contribution readiness means trust. Tests and clear communication are part of the code.

## Public Meetup Mode

For meetup or study-group use:

- keep secrets and real wallets out of scope
- use regtest, signet, or toy models
- start with runnable katas
- make everyone run tests locally
- explain Bitcoin terms through code, not slides alone
- end with one official-docs link and one next kata

## Repository Convention

BDK Dojo is one growing practice repo plus one public curriculum/example repo.

Public curriculum repo:

```text
bdk-dojo-skill/
  scaffold/
    1.1-amounts-and-utxos/
      README.md
      stubs.rs
  examples/
    bdk-dojo-wallet/
      Cargo.toml
      src/
        amount.rs
        utxo.rs
        balance.rs
        lib.rs
        main.rs
      tests/
        wallet_primitives.rs
```

For learner practice repos:

```text
bdk-dojo/
  README.md
  Cargo.toml
  src/
    lib.rs
    amount.rs
    utxo.rs
    balance.rs
    wallet.rs
    sync.rs
    fees.rs
    coin_selection.rs
    tx_plan.rs
    psbt_review.rs
    descriptors.rs
  tests/
    tests.rs
    wallet_flow.rs
    balance/
    sync/
    spending/
    bdk_bridge/
  lessons/
    completed.md
  notes/
    progress.md
    pain-points.md
    questions.md
    bdk-bridge.md
```

Default assumption:

- keep using the existing `bdk-dojo` learner repo once it exists
- use `examples/bdk-dojo-wallet/` as the public working reference crate
- add each kata as a clearly named module/function in the relevant domain file, not as a pile in `src/lib.rs`
- add or extend tests for that kata instead of deleting prior verified lessons
- only start a fresh repo when the user explicitly asks for a reset
- keep toy lessons separated with clear names so the repo stays readable

Use this as the training yard. A real BDK fork comes later for upstream contribution work.

## Common Pitfalls

- Do not turn the lesson into passive reading.
- Do not write the learner's first full solution.
- Do not touch real keys, seeds, wallets, or funds.
- Do not confuse compiler green with correctness.
- Do not introduce BDK APIs before the Rust and Bitcoin model are ready.
- Do not make every lesson huge. Small katas compound.
- Do not promise grant-readiness from a few exercises. Build proof over time.
- Do not present toy models as production wallet logic.

## Verification Checklist

- [ ] One concept only.
- [ ] Learner writes the core logic first.
- [ ] At least one normal test exists.
- [ ] At least one edge-case test exists.
- [ ] `cargo test` passes.
- [ ] Expected behavior from the scaffold is verified.
- [ ] Reference crate comparison happens after the learner attempt when useful.
- [ ] Warnings are reviewed, not ignored.
- [ ] Bitcoin model is explained accurately.
- [ ] Rust concept/pain point is named.
- [ ] BDK bridge note exists once the toy concept is stable.
- [ ] Learner can explain the concept in one or two sentences.
- [ ] Real keys/funds are not involved.
- [ ] Progress journal entry is drafted or updated.
- [ ] Next kata is clear.
