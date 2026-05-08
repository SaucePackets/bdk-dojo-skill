# Wallet Balance UTXO Model

Use this as the first BDK Dojo lesson when the learner is starting from Rust project basics.

## Lesson intent

Teach Rust project shape while introducing the balance buckets wallet libraries need to reason about.

## Setup

Default: continue in the existing `bdk-dojo` repo.

Only create it if it does not exist yet:

```bash
cargo new bdk-dojo
cd bdk-dojo
```

For later lessons, add the kata to the existing `src/lib.rs` unless the lesson explicitly says otherwise.
Do not delete prior working katas; keep them named and tested.

Explain:

- `Cargo.toml`: package config and dependencies.
- `src/main.rs`: binary entry point.
- `src/lib.rs`: reusable logic and unit tests.
- Crate import trap: package `bdk-dojo` imports as `bdk_dojo`.

## Starter structs

```rust
#[derive(Debug, Clone, Copy)]
pub struct Utxo {
    pub value: u64,
    pub confirmed: bool,
    pub spendable: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BalanceSummary {
    pub confirmed: u64,
    pub trusted_pending: u64,
    pub untrusted_pending: u64,
    pub total_spendable: u64,
}
```

## Classification rules

- Confirmed + spendable -> `confirmed`.
- Unconfirmed + spendable -> `trusted_pending`.
- Unconfirmed + not spendable -> `untrusted_pending`.
- `total_spendable = confirmed + trusted_pending`.
- Confirmed but not spendable is ignored under this beginner model.

Correct branch shape:

```rust
for utxo in utxos {
    if utxo.confirmed && utxo.spendable {
        confirmed += utxo.value;
    } else if !utxo.confirmed && utxo.spendable {
        trusted_pending += utxo.value;
    } else if !utxo.confirmed && !utxo.spendable {
        untrusted_pending += utxo.value;
    }
}

let total_spendable = confirmed + trusted_pending;
```

## Required test

Use `#[cfg(test)] mod tests` at the bottom of `src/lib.rs`, `use super::*;`, and an `assert_eq!` against `BalanceSummary`.

Expected sample:

```text
confirmed: 50_000
trusted_pending: 20_000
untrusted_pending: 10_000
total_spendable: 70_000
```

## Common pitfall

A version can compile and run while still being wrong. A duplicated condition can skip the unconfirmed/spendable branch. Treat compiler success, warnings, tests, and runtime output as separate signals.

## Verification

```bash
cargo test
cargo run
```

Review output and warnings. If Rust warns a variable assignment is never read, derive the value once:

```rust
let total_spendable = confirmed + trusted_pending;
```

## OSS connection

Wallet balance display bugs are user-trust bugs. Even this toy model trains the discipline maintainers expect: explicit state buckets, tests for classification, and clear naming around spendability.
