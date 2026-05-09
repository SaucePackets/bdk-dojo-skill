## Setup

- Work in the cumulative crate: `examples/bdk-dojo-wallet/` or your learner `bdk-dojo/` repo.
- Create or update: `src/bdk_bridge.rs`.
- Copy the stubs from `scaffold/5.1-bdk-project-orientation/stubs.rs` into your codebase when you reach this lesson.
- Write your solution.
- Run: `cargo test`.

## Goal

Learn how to approach BDK before touching upstream code.

## Builds on

- toy wallet model

## Expected behavior

- identify docs/examples before issues
- prefer tests/docs/examples before core rewrites

## Required tests

- `bdk_bridge_notes_name_what_the_toy_model_hides`

## Reference implementation

- `examples/bdk-dojo-wallet/src/bdk_bridge.rs`

Only compare to the reference after the learner attempts the lesson.

## BDK bridge

- This toy exercise models one wallet-engineering concept. Real BDK includes descriptors, chain sources, persistence, transaction graph state, and stricter policy boundaries.
- Use BDK docs/examples before claiming exact API details.

## Done when

- identify docs/examples before issues
- prefer tests/docs/examples before core rewrites
- Required tests pass with `cargo test`.
- The learner can explain the concept in one or two sentences.
