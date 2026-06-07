# 1.1 Amounts And UTXOs

## Setup

- Work in the cumulative reference crate: `examples/bdk-dojo-wallet/`, or in your own learner repo/fork. Do not assume the repo is named `bdk-dojo`.
- Create or update: `src/amount.rs` and `src/utxo.rs`.
- Update `src/lib.rs` to expose both modules.
- Copy the stubs from `scaffold/1.1-amounts-and-utxos/stubs.rs` into your codebase.
- Copy the tests from `scaffold/1.1-amounts-and-utxos/tests.rs` into `tests/lesson_1_1_amounts_and_utxos.rs`.
- Update the `use` import in `tests.rs` to match your Cargo.toml package name.
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

Defined in `scaffold/1.1-amounts-and-utxos/tests.rs`:

- `amount_preserves_sats_exactly`
- `utxo_stores_outpoint_and_value`

Copy the test file into your project's `tests/` directory. The tests encode the expected behavior — `cargo test` is the pass/fail gate.

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
