# 3.4 Transaction Proposal

## Setup

- Update `src/lib.rs` with the needed module exports:
  - `pub mod tx_plan;`
  - `pub use tx_plan::{TxPlan, propose_transaction};`

- Work in the cumulative reference crate: `examples/bdk-dojo-wallet/`, or in your own learner repo/fork. Do not assume the repo is named `bdk-dojo`.
- Create or update: `src/tx_plan.rs`.
- Copy the stubs from `scaffold/3.4-transaction-proposal/stubs.rs` into your codebase when you reach this lesson.
- Write your solution.
- Run: `cargo test`.

## Goal

Create an unsigned transaction plan before any signing happens.

## Builds on

- coin selection
- fee rates
- change policy

## Expected behavior

- plan contains selected inputs, recipient, amount, fee, and change decision
- insufficient funds are reported before signing

## Required tests

- `transaction_proposal_contains_selected_inputs_fee_and_change`

## Reference implementation

- `examples/bdk-dojo-wallet/src/tx_plan.rs`

Only compare to the reference after the learner attempts the lesson.

## BDK bridge

- This toy exercise models one wallet-engineering concept. Real BDK includes descriptors, chain sources, persistence, transaction graph state, and stricter policy boundaries.
- Use BDK docs/examples before claiming exact API details.

## Done when

- plan contains selected inputs, recipient, amount, fee, and change decision
- insufficient funds are reported before signing
- Required tests pass with `cargo test`.
- The learner can explain the concept in one or two sentences.
