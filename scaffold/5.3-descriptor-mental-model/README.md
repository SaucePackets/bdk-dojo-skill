## Setup

- Update `src/lib.rs` with the needed module exports:
  - `pub mod descriptors;`
  - `pub use descriptors::{DescriptorKind, classify_descriptor, validate_toy_descriptor};`

- Work in the cumulative reference crate: `examples/bdk-dojo-wallet/`, or in your own learner repo/fork. Do not assume the repo is named `bdk-dojo`.
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
