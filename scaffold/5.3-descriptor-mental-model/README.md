## Setup

- Work in the cumulative crate: `examples/bdk-dojo-wallet/` or your learner `bdk-dojo/` repo.
- Create or update: `src/descriptors.rs`.
- Copy the stubs from `scaffold/5.3-descriptor-mental-model/stubs.rs` into your codebase when you reach this lesson.
- Write your solution.
- Run: `cargo test`.

## Goal

Treat descriptors as wallet policy descriptions without parsing real secrets.

## Builds on

- wallet policy
- BDK bridge

## Expected behavior

- wpkh-like descriptors classify as single-sig
- sortedmulti-like descriptors classify as multisig
- unknown strings are invalid

## Required tests

- `descriptor_classifier_recognizes_common_policy_shapes`

## Reference implementation

- `examples/bdk-dojo-wallet/src/descriptors.rs`

Only compare to the reference after the learner attempts the lesson.

## BDK bridge

- This toy exercise models one wallet-engineering concept. Real BDK includes descriptors, chain sources, persistence, transaction graph state, and stricter policy boundaries.
- Use BDK docs/examples before claiming exact API details.

## Done when

- wpkh-like descriptors classify as single-sig
- sortedmulti-like descriptors classify as multisig
- unknown strings are invalid
- Required tests pass with `cargo test`.
- The learner can explain the concept in one or two sentences.
