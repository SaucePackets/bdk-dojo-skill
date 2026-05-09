## Setup

- Merge these fields into `src/utxo.rs` on `Utxo`:

```rust
pub confirmed: bool,
pub spendable: bool,
```

- Work in the cumulative reference crate: `examples/bdk-dojo-wallet/`, or in your own learner repo/fork. Do not assume the repo is named `bdk-dojo`.
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

## Expected behavior

Given:

```text
confirmed + spendable = 50_000
unconfirmed + spendable = 20_000
unconfirmed + unspendable = 10_000
confirmed + unspendable = 5_000
```

Expected summary:

```text
confirmed = 50_000
trusted_pending = 20_000
untrusted_pending = 10_000
total_spendable = 70_000
```

Confirmed but unspendable is ignored in this beginner model.

## Required tests

- `classify_balance_separates_trust_and_spendability`
- `classify_balance_empty_wallet_is_zeroed`

## Reference implementation

- `examples/bdk-dojo-wallet/src/balance.rs`

Only compare to the reference after the learner attempts the lesson.

## BDK bridge

- Toy balance buckets introduce why wallets expose categories. Real BDK derives categories from confirmation, trust, transaction graph, and spendability policy.

## Done when

- Confirmed spendable sats go into `confirmed`.
- Unconfirmed spendable sats go into `trusted_pending`.
- Unconfirmed unspendable sats go into `untrusted_pending`.
- `total_spendable = confirmed + trusted_pending`.
- You did not just rename `calculate_balance`.
