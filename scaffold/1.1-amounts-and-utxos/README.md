## Setup

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

## Done when

- You have an `Amount` wrapper around sats.
- You have `OutPoint` and `Utxo` structs.
- Tests prove sats are preserved exactly.
- No real wallet secrets exist anywhere.
