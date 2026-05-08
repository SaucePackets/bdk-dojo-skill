## Setup

- Work in the cumulative crate: `examples/bdk-dojo-wallet/` or your learner `bdk-dojo/` repo.
- Update: `src/utxo.rs` so UTXOs carry confirmation and spendability state.
- Update: `src/balance.rs`.
- Copy the stubs from `scaffold/1.3-balance-buckets/stubs.rs` into your codebase.
- Write your solution.
- Run: `cargo test`.

## Goal

Classify wallet sats by trust and spendability.

This is different from `calculate_balance`:

- `calculate_balance` asks: “how many sats total?”
- `classify_balance` asks: “what kind of sats?”

## Builds on

- `Amount`
- `Utxo`
- `calculate_balance`

## Done when

- Confirmed spendable sats go into `confirmed`.
- Unconfirmed spendable sats go into `trusted_pending`.
- Unconfirmed unspendable sats go into `untrusted_pending`.
- `total_spendable = confirmed + trusted_pending`.
- You did not just rename `calculate_balance`.
