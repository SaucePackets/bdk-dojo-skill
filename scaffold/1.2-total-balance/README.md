## Setup

- Work in the cumulative crate: `examples/bdk-dojo-wallet/` or your learner `bdk-dojo/` repo.
- Create or update: `src/balance.rs`.
- Update `src/lib.rs` to expose `amount`, `utxo`, and `balance`.
- Copy the stubs from `scaffold/1.2-total-balance/stubs.rs` into your codebase.
- Write your solution.
- Run: `cargo test`.

## Goal

Calculate the plain total sats controlled by a list of UTXOs.

This lesson answers one question only: “how many sats total?”

## Builds on

- `Amount`
- `Utxo`

## Expected behavior

- `calculate_balance(&[])` returns `0`.
- `calculate_balance(&utxos)` returns the sum of every UTXO value in sats.
- Confirmed, pending, spendable, and unspendable flags do not affect this total-only function.

## Required tests

- `calculate_balance_empty_wallet_is_zero`
- `calculate_balance_sums_all_utxos`

## Reference implementation

- `examples/bdk-dojo-wallet/src/balance.rs`

Only compare to the reference after the learner attempts the lesson.

## Done when

- `calculate_balance(&utxos)` returns total sats.
- Empty wallet returns zero.
- This function does not classify trust, confirmations, or spendability.
