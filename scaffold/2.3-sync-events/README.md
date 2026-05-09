## Setup

- Work in the cumulative crate: `examples/bdk-dojo-wallet/` or your learner `bdk-dojo/` repo.
- Create or update: `src/wallet.rs`.
- Copy the stubs from `scaffold/2.3-sync-events/stubs.rs` into your codebase when you reach this lesson.
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

- `wallet_apply_tracks_found_confirmed_spent_and_reorged_utxos`

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
