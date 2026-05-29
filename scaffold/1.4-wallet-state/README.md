# 1.4 Wallet State

## Setup

- Work in the cumulative reference crate: `examples/bdk-dojo-wallet/`, or in your own learner repo/fork. Do not assume the repo is named `bdk-dojo`.
- Create: `src/wallet.rs`.
- Update: `src/lib.rs` to expose `wallet` and `WalletState`.
- Copy the stubs from `scaffold/1.4-wallet-state/stubs.rs` into your codebase.
- Write your solution.
- Run: `cargo test`.

## Goal

Wrap wallet-owned UTXOs in a small wallet domain object.

This lesson answers: “where does wallet state live?”

`classify_balance(&utxos)` is still the balance logic. `WalletState::balance()` should delegate to it instead of duplicating the bucket rules.

## Builds on

- `Utxo`
- `BalanceSummary`
- `classify_balance`

## Expected behavior

Given a wallet with:

```text
confirmed + spendable = 50_000
unconfirmed + spendable = 20_000
unconfirmed + unspendable = 10_000
```

Expected:

```text
wallet.balance().confirmed = 50_000
wallet.balance().trusted_pending = 20_000
wallet.balance().untrusted_pending = 10_000
wallet.balance().total_spendable = 70_000
```

Given an empty wallet:

```text
wallet.balance() returns all zero buckets
```

## Required tests

- `wallet_balance_delegates_to_classify_balance`
- `empty_wallet_balance_is_zeroed`

## Reference implementation

- `examples/bdk-dojo-wallet/src/wallet.rs`

Only compare to the reference after the learner attempts the lesson.

## BDK bridge

- Toy `WalletState` introduces domain boundaries. Real BDK wallet state includes descriptors, local chain checkpoints, indexed transactions, persistence, and sync state.

## Done when

- `WalletState` owns `utxos`.
- `WalletState` stores `tip_height` for later confirmation-depth lessons.
- `WalletState::balance()` returns `BalanceSummary`.
- `WalletState::balance()` calls `classify_balance(&self.utxos)`.
- You did not copy the classification `if` chain into `wallet.rs`.
