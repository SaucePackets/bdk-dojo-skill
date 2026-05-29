# 4.3 Full Toy Send Flow

## Setup

- Work in the cumulative reference crate: `examples/bdk-dojo-wallet/`, or in your own learner repo/fork. Do not assume the repo is named `bdk-dojo`.
- Create or update: `tests/wallet_flow.rs`.
- Copy the stubs from `scaffold/4.3-full-toy-send-flow/stubs.rs` into your codebase when you reach this lesson.
- Write your solution.
- Run: `cargo test`.

## Goal

Prove the full toy send flow works across sync, balance, coin selection, tx planning, and review.

## Builds on

- sync events
- balance
- coin selection
- tx planning
- PSBT review

## Expected behavior

- wallet finds funds
- balance updates
- proposal is created
- review approves safe plan

## Required tests

- `full_toy_send_flow_can_be_reviewed_before_signing`

## Reference implementation

- `examples/bdk-dojo-wallet/tests/wallet_flow.rs`

Only compare to the reference after the learner attempts the lesson.

## BDK bridge

- This toy exercise models one wallet-engineering concept. Real BDK includes descriptors, chain sources, persistence, transaction graph state, and stricter policy boundaries.
- Use BDK docs/examples before claiming exact API details.

## Done when

- wallet finds funds
- balance updates
- proposal is created
- review approves safe plan
- Required tests pass with `cargo test`.
- The learner can explain the concept in one or two sentences.
