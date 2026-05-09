## Setup

- Work in the cumulative reference crate: `examples/bdk-dojo-wallet/`, or in your own learner repo/fork. Do not assume the repo is named `bdk-dojo`.
- Create or update: `notes/capstone-wallet-flow.md`.
- Use the stubs from `scaffold/5.7-capstone-wallet-flow/stubs.rs` into your codebase when you reach this lesson.
- Write the Markdown artifact in `notes/capstone-wallet-flow.md`.
- The Rust stub is a checklist/reference helper, not required production wallet code.
- Run: `cargo test` to ensure the existing reference crate still passes.

## Goal

Explain the full wallet flow from UTXO discovery to BDK mapping.

## Builds on

- all prior lessons

## Expected behavior

- explain UTXO -> sync -> balance -> selection -> plan -> review -> BDK mapping
- name toy limitations honestly

## Required tests

- `write a capstone explanation and compare to BDK bridge notes`

## Reference implementation

- `scaffold/5.7-capstone-wallet-flow/stubs.rs`

Only compare to the reference after the learner attempts the lesson.

## BDK bridge

- This toy exercise models one wallet-engineering concept. Real BDK includes descriptors, chain sources, persistence, transaction graph state, and stricter policy boundaries.
- Use BDK docs/examples before claiming exact API details.

## Done when

- explain UTXO -> sync -> balance -> selection -> plan -> review -> BDK mapping
- name toy limitations honestly
- Existing reference tests pass with `cargo test`, and the Markdown artifact satisfies the review prompt.
- The learner can explain the concept in one or two sentences.
