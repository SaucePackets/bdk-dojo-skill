## Setup

- Work in the cumulative crate: `examples/bdk-dojo-wallet/` or your learner `bdk-dojo/` repo.
- Create or update: `src/wallet.rs`.
- Copy the stubs from `scaffold/2.4-checkpoints-and-reorgs/stubs.rs` into your codebase when you reach this lesson.
- Write your solution.
- Run: `cargo test`.

## Goal

Roll wallet chain state back to a prior height after a toy reorg.

## Builds on

- WalletState::checkpoints
- SyncEvent::TipAdvanced

## Expected behavior

- tip height moves backward
- checkpoints above rollback height are removed
- UTXOs confirmed above rollback height become unconfirmed

## Required tests

- `rollback_unconfirms_utxos_above_new_tip`

## Reference implementation

- `examples/bdk-dojo-wallet/src/wallet.rs`

Only compare to the reference after the learner attempts the lesson.

## BDK bridge

- This toy exercise models one wallet-engineering concept. Real BDK includes descriptors, chain sources, persistence, transaction graph state, and stricter policy boundaries.
- Use BDK docs/examples before claiming exact API details.

## Done when

- tip height moves backward
- checkpoints above rollback height are removed
- UTXOs confirmed above rollback height become unconfirmed
- Required tests pass with `cargo test`.
- The learner can explain the concept in one or two sentences.
