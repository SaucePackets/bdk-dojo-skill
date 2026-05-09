## Setup

- Work in the cumulative crate: `examples/bdk-dojo-wallet/` or your learner `bdk-dojo/` repo.
- Create or update: `src/amount.rs` and `src/utxo.rs`.
- Update `src/lib.rs` to expose both modules.
- Copy the stubs from `scaffold/1.1-amounts-and-utxos/stubs.rs` into your codebase.
- Write your solution.
- Run: `cargo test`.

## Goal

Model wallet money as sats and wallet coins as UTXOs.

No floats. No real keys. Just the data shape wallet code needs before BDK enters the room.

## Builds on

- Fresh Rust crate.

## Expected behavior

- `Amount::from_sats(n).to_sats()` returns exactly `n`.
- `OutPoint` stores a transaction id and output index.
- `Utxo` stores an outpoint and exact sat value.

## Required tests

- `amount_preserves_sats_exactly`
- `utxo_stores_outpoint_and_value`

## Reference implementation

- `examples/bdk-dojo-wallet/src/amount.rs`
- `examples/bdk-dojo-wallet/src/utxo.rs`

Only compare to the reference after the learner attempts the lesson.

## BDK bridge

- Toy `Amount`, `OutPoint`, and `Utxo` map to real Bitcoin transaction outputs, but real BDK also tracks scripts, chain position, keychain, and transaction graph state.

## Done when

- You have an `Amount` wrapper around sats.
- You have `OutPoint` and `Utxo` structs.
- Tests prove sats are preserved exactly.
- No real wallet secrets exist anywhere.
