## Setup

- Work in the cumulative crate: `examples/bdk-dojo-wallet/` or your learner `bdk-dojo/` repo.
- Create or update: `notes/bdk-wallet-skeleton.md`.
- Copy the stubs from `scaffold/5.4-bdk-wallet-skeleton/stubs.rs` into your codebase when you reach this lesson.
- Write your solution.
- Run: `cargo test`.

## Goal

Sketch a safe regtest/signet-only BDK wallet skeleton without real funds.

## Builds on

- descriptor mental model

## Expected behavior

- uses example descriptors only
- names regtest/signet safety boundary
- does not require real seed phrases or funds

## Required tests

- `read README and keep code as safe pseudocode`

## Reference implementation

- `scaffold/5.4-bdk-wallet-skeleton/stubs.rs`

Only compare to the reference after the learner attempts the lesson.

## BDK bridge

- This toy exercise models one wallet-engineering concept. Real BDK includes descriptors, chain sources, persistence, transaction graph state, and stricter policy boundaries.
- Use BDK docs/examples before claiming exact API details.

## Done when

- uses example descriptors only
- names regtest/signet safety boundary
- does not require real seed phrases or funds
- Required tests pass with `cargo test`.
- The learner can explain the concept in one or two sentences.
