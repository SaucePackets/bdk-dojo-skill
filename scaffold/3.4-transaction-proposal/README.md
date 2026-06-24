# 3.4 Transaction Proposal

## Setup

- Update `src/lib.rs` with the needed module exports:
  - `pub mod tx_plan;`
  - `pub use tx_plan::{TxPlan, propose_transaction};`

- Work in the cumulative reference crate: `examples/bdk-dojo-wallet/`, or in your own learner repo/fork.
- Learner crate convention: package name `bdk-dojo`, Rust import name `bdk_dojo`.
- Create or update: `src/tx_plan.rs`.
- Copy the stubs from `scaffold/3.4-transaction-proposal/stubs.rs` into your codebase when you reach this lesson.
- Copy the tests from `scaffold/3.4-transaction-proposal/lesson_3_4_transaction_proposal.rs` into `tests/lesson_3_4_transaction_proposal.rs`.
- Keep the test import as `bdk_dojo`; the Cargo package should be named `bdk-dojo` (hyphen in package, underscore in Rust import).
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

Defined in `scaffold/3.4-transaction-proposal/lesson_3_4_transaction_proposal.rs`:

- `transaction_proposal_produces_valid_plan`
- `transaction_proposal_fails_with_insufficient_funds`

Copy the test file into your project's `tests/` directory. The tests encode the expected behavior — `cargo test` is the pass/fail gate.

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
