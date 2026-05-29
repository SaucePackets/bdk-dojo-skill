# 4.1 PSBT Review

## Setup

- Extend `WalletError` in `src/errors.rs` with:

```rust
UnsafePsbt(String),
UnknownRecipient(String),
```

- Update `src/lib.rs` with the needed module exports:
  - `pub mod psbt_review;`
  - `pub use psbt_review::{PsbtReview, WalletPolicy, review_plan};`

- Work in the cumulative reference crate: `examples/bdk-dojo-wallet/`, or in your own learner repo/fork. Do not assume the repo is named `bdk-dojo`.
- Create or update: `src/psbt_review.rs`.
- Copy the stubs from `scaffold/4.1-psbt-review/stubs.rs` into your codebase when you reach this lesson.
- Write your solution.
- Run: `cargo test`.

## Goal

Review a transaction plan before signing like a PSBT review checklist.

## Builds on

- TxPlan

## Expected behavior

- unknown recipients are rejected
- fees above policy are rejected
- safe plans are approved

## Required tests

- `psbt_review_rejects_unknown_recipient_and_high_fee`

## Reference implementation

- `examples/bdk-dojo-wallet/src/psbt_review.rs`

Only compare to the reference after the learner attempts the lesson.

## BDK bridge

- This toy exercise models one wallet-engineering concept. Real BDK includes descriptors, chain sources, persistence, transaction graph state, and stricter policy boundaries.
- Use BDK docs/examples before claiming exact API details.

## Done when

- unknown recipients are rejected
- fees above policy are rejected
- safe plans are approved
- Required tests pass with `cargo test`.
- The learner can explain the concept in one or two sentences.
