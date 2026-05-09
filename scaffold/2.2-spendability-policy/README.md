## Setup

- Update `src/lib.rs` with the needed module exports:
  - `pub use chain::{COINBASE_MATURITY, is_spendable};`

- Merge these fields into `src/utxo.rs` on `Utxo`:

```rust
pub coinbase: bool,
pub locked_until: Option<u32>,
pub owned: bool,
```

- Existing normal wallet UTXOs should use `coinbase: false`, `locked_until: None`, and `owned: true`.

- Work in the cumulative reference crate: `examples/bdk-dojo-wallet/`, or in your own learner repo/fork. Do not assume the repo is named `bdk-dojo`.
- Create or update: `src/chain.rs`.
- Copy the stubs from `scaffold/2.2-spendability-policy/stubs.rs` into your codebase when you reach this lesson.
- Write your solution.
- Run: `cargo test`.

## Goal

Decide whether a toy UTXO is actually spendable policy-wise.

## Builds on

- confirmations
- Utxo ownership and lock fields

## Expected behavior

- foreign UTXOs are not spendable
- locked UTXOs are not spendable before lock height
- immature coinbase UTXOs are not spendable before 100 confirmations

## Required tests

- `spendability_rejects_immature_coinbase_locked_and_foreign_utxos`

## Reference implementation

- `examples/bdk-dojo-wallet/src/chain.rs`

Only compare to the reference after the learner attempts the lesson.

## BDK bridge

- This toy exercise models one wallet-engineering concept. Real BDK includes descriptors, chain sources, persistence, transaction graph state, and stricter policy boundaries.
- Use BDK docs/examples before claiming exact API details.

## Done when

- foreign UTXOs are not spendable
- locked UTXOs are not spendable before lock height
- immature coinbase UTXOs are not spendable before 100 confirmations
- Required tests pass with `cargo test`.
- The learner can explain the concept in one or two sentences.
