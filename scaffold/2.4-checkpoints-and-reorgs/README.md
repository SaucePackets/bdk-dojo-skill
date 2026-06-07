# 2.4 Checkpoints And Reorgs

## Setup

- Merge this field into `src/wallet.rs` on `WalletState`:

```rust
pub checkpoints: Vec<u32>,
```

- Update `WalletState::new` so it initializes `checkpoints` with the starting `tip_height`.

- Work in the cumulative reference crate: `examples/bdk-dojo-wallet/`, or in your own learner repo/fork. Do not assume the repo is named `bdk-dojo`.
- Create or update: `src/wallet.rs`.
- Copy the stubs from `scaffold/2.4-checkpoints-and-reorgs/stubs.rs` into your codebase when you reach this lesson.
- Copy the tests from `scaffold/2.4-checkpoints-and-reorgs/tests.rs` into your project's `tests/` directory.
- Update the `use` import in `tests.rs` to match your Cargo.toml package name.
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

Defined in `scaffold/2.4-checkpoints-and-reorgs/tests.rs`:

- `rollback_unconfirms_utxos_above_new_tip`

Copy the test file into your project's `tests/` directory. The tests encode the expected behavior — `cargo test` is the pass/fail gate.

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
