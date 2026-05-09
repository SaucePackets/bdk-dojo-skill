## Setup

- Work in the cumulative reference crate: `examples/bdk-dojo-wallet/`, or in your own learner repo/fork. Do not assume the repo is named `bdk-dojo`.
- Create or update: `src/errors.rs`.
- Copy the stubs from `scaffold/4.2-error-handling-pass/stubs.rs` into your codebase when you reach this lesson.
- Write your solution.
- Run: `cargo test`.

## Goal

Replace vague booleans and options with maintainer-friendly wallet errors.

## Builds on

- coin selection
- tx planning
- PSBT review

## Expected behavior

- insufficient funds returns WalletError::InsufficientFunds
- unsafe PSBT review returns specific error variants

## Required tests

- `coin_selection_reports_insufficient_funds`
- `psbt_review_rejects_unknown_recipient_and_high_fee`

## Reference implementation

- `examples/bdk-dojo-wallet/src/errors.rs`

Only compare to the reference after the learner attempts the lesson.

## BDK bridge

- This toy exercise models one wallet-engineering concept. Real BDK includes descriptors, chain sources, persistence, transaction graph state, and stricter policy boundaries.
- Use BDK docs/examples before claiming exact API details.

## Done when

- insufficient funds returns WalletError::InsufficientFunds
- unsafe PSBT review returns specific error variants
- Required tests pass with `cargo test`.
- The learner can explain the concept in one or two sentences.
