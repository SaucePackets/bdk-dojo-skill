## Setup

- Update `src/lib.rs` with the needed module exports:
  - `pub mod fees;`
  - `pub use fees::{FeeRate, TxSizeEstimate, fee};`

- Work in the cumulative crate: `examples/bdk-dojo-wallet/` or your learner `bdk-dojo/` repo.
- Create or update: `src/fees.rs`.
- Copy the stubs from `scaffold/3.1-fee-rates-and-vbytes/stubs.rs` into your codebase when you reach this lesson.
- Write your solution.
- Run: `cargo test`.

## Goal

Calculate transaction fees from virtual size and sat/vB fee rate.

## Builds on

- wallet balance and spend planning

## Expected behavior

- 141 vbytes at 2 sat/vB costs 282 sats
- fee is based on transaction size, not sent amount

## Required tests

- `fee_is_vbytes_times_fee_rate`

## Reference implementation

- `examples/bdk-dojo-wallet/src/fees.rs`

Only compare to the reference after the learner attempts the lesson.

## BDK bridge

- This toy exercise models one wallet-engineering concept. Real BDK includes descriptors, chain sources, persistence, transaction graph state, and stricter policy boundaries.
- Use BDK docs/examples before claiming exact API details.

## Done when

- 141 vbytes at 2 sat/vB costs 282 sats
- fee is based on transaction size, not sent amount
- Required tests pass with `cargo test`.
- The learner can explain the concept in one or two sentences.
