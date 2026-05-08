## Setup

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

## Done when

- `calculate_balance(&utxos)` returns total sats.
- Empty wallet returns zero.
- This function does not classify trust, confirmations, or spendability.
