# 2.3 Sync Events

## Setup

- Update `src/lib.rs` with the needed module exports:
  - `pub use wallet::SyncEvent;`

- Work in the cumulative reference crate: `examples/bdk-dojo-wallet/`, or in your own learner repo/fork.
- Learner crate convention: package name `bdk-dojo`, Rust import name `bdk_dojo`.
- Create or update: `src/wallet.rs`.
- Copy the stubs from `scaffold/2.3-sync-events/stubs.rs` into your codebase when you reach this lesson.
- Copy the tests from `scaffold/2.3-sync-events/lesson_2_3_sync_events.rs` into `tests/lesson_2_3_sync_events.rs`.
- Keep the test import as `bdk_dojo`; the Cargo package should be named `bdk-dojo` (hyphen in package, underscore in Rust import).
- Write your solution.
- Run: `cargo test`.

## Goal

Apply wallet sync events to mutate wallet state over time.

## Builds on

- WalletState
- Utxo

## Expected behavior

- Found adds a UTXO
- Confirmed sets seen_at_height and confirmed
- Reorged unconfirms a UTXO
- Spent removes a UTXO

## Required tests

Defined in `scaffold/2.3-sync-events/lesson_2_3_sync_events.rs`:

- `wallet_apply_tracks_found_confirmed_spent_and_reorged_utxos`

Copy the test file into your project's `tests/` directory. The tests encode the expected behavior — `cargo test` is the pass/fail gate.

## Reference implementation

- `examples/bdk-dojo-wallet/src/wallet.rs`

Only compare to the reference after the learner attempts the lesson.

## BDK bridge

- This toy exercise models one wallet-engineering concept. Real BDK includes descriptors, chain sources, persistence, transaction graph state, and stricter policy boundaries.
- Use BDK docs/examples before claiming exact API details.

## Done when

- Found adds a UTXO
- Confirmed sets seen_at_height and confirmed
- Reorged unconfirms a UTXO
- Spent removes a UTXO
- Required tests pass with `cargo test`.
- The learner can explain the concept in one or two sentences.
