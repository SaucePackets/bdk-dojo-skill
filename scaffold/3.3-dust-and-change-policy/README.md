## Setup

- Update `src/lib.rs` with the needed module exports:
  - `pub mod change;`
  - `pub use change::{ChangeDecision, DUST_LIMIT, decide_change};`

- Work in the cumulative reference crate: `examples/bdk-dojo-wallet/`, or in your own learner repo/fork. Do not assume the repo is named `bdk-dojo`.
- Create or update: `src/change.rs`.
- Copy the stubs from `scaffold/3.3-dust-and-change-policy/stubs.rs` into your codebase when you reach this lesson.
- Write your solution.
- Run: `cargo test`.

## Goal

Decide whether leftover sats should become change or be added to fee as dust.

## Builds on

- coin selection leftover amount

## Expected behavior

- 0 leftover has no change
- leftover below DUST_LIMIT is added to fee
- leftover at or above DUST_LIMIT becomes change

## Required tests

- `dust_change_is_added_to_fee_instead_of_output`

## Reference implementation

- `examples/bdk-dojo-wallet/src/change.rs`

Only compare to the reference after the learner attempts the lesson.

## BDK bridge

- This toy exercise models one wallet-engineering concept. Real BDK includes descriptors, chain sources, persistence, transaction graph state, and stricter policy boundaries.
- Use BDK docs/examples before claiming exact API details.

## Done when

- 0 leftover has no change
- leftover below DUST_LIMIT is added to fee
- leftover at or above DUST_LIMIT becomes change
- Required tests pass with `cargo test`.
- The learner can explain the concept in one or two sentences.
