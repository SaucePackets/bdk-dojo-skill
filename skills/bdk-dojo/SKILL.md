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

## Default Lesson Flow

1. Pick one Bitcoin primitive or BDK-shaped concept.
2. Explain why it matters in wallet engineering.
3. Give a tiny Rust exercise.
4. Make the learner implement the core logic first.
5. Require at least one test.
6. Verify with `cargo test` and, when relevant, `cargo run`.
7. Review compiler success, warnings, correctness, and code clarity separately.
8. Tie the lesson back to real BDK or open-source contribution habits.

## Lesson Prompt Format

```text
Lane:
Concept:
Rust focus:
Difficulty:
Problem:
Rules:
Examples:
Your first move:
Verification:
OSS connection:
```

## Review Format

```text
Verdict:
Correctness:
Rust clarity:
Bitcoin model:
Tests:
One improvement:
Next kata:
```

Keep feedback short and direct. No academic fog machine.

## Beginner Track

Start with Rust comfort and wallet primitives:

- Cargo project layout: `Cargo.toml`, `src/lib.rs`, `src/main.rs`, tests.
- Crate naming: package `bdk-dojo` imports as `bdk_dojo`.
- UTXO balance buckets.
- Confirmed vs pending funds.
- Spendable vs unspendable outputs.
- Basic error handling with `Result`.
- Simple structs and enums for wallet state.

First recommended lesson: `references/wallet-balance-utxo-model.md`.

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

BDK Dojo is normally one growing practice repo, not a fresh throwaway repo per lesson.

For learner practice repos:

```text
bdk-dojo/
  README.md
  Cargo.toml
  src/
    lib.rs
    main.rs
  lessons/
  notes/
```

Default assumption:

- keep using the existing `bdk-dojo` repo once it exists
- add each kata as a clearly named function/module in `src/lib.rs`
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
- [ ] At least one test exists.
- [ ] `cargo test` passes.
- [ ] Warnings are reviewed, not ignored.
- [ ] Bitcoin model is explained accurately.
- [ ] Real keys/funds are not involved.
- [ ] Next kata is clear.
