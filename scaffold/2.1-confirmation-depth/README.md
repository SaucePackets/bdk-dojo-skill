## Setup

- Update `src/lib.rs` with the needed module exports:
  - `pub mod chain;`
  - `pub use chain::confirmations;`

- Merge this field into `src/utxo.rs` on `Utxo`:

```rust
pub seen_at_height: Option<u32>,
```

- Update existing UTXO test fixtures to set `seen_at_height`. Confirmed UTXOs should usually use `Some(height)`; mempool UTXOs should use `None`.

- Work in the cumulative reference crate: `examples/bdk-dojo-wallet/`, or in your own learner repo/fork. Do not assume the repo is named `bdk-dojo`.
- Create or update: `src/chain.rs`.
- Copy the stubs from `scaffold/2.1-confirmation-depth/stubs.rs` into your codebase when you reach this lesson.
- Write your solution.
- Run: `cargo test`.

## Goal

Calculate confirmation depth from a UTXO seen height and the current chain tip.

## Builds on

- WalletState::tip_height
- Utxo::seen_at_height

## Expected behavior

- confirmed at height 100 with tip 100 has 1 confirmation
- confirmed at height 100 with tip 105 has 6 confirmations
- mempool UTXO with no seen height has 0 confirmations

## Required tests

- `confirmations_count_from_seen_height_to_tip`
- `mempool_utxo_has_zero_confirmations`

## Reference implementation

- `examples/bdk-dojo-wallet/src/chain.rs`

Only compare to the reference after the learner attempts the lesson.

## BDK bridge

- This toy exercise models one wallet-engineering concept. Real BDK includes descriptors, chain sources, persistence, transaction graph state, and stricter policy boundaries.
- Use BDK docs/examples before claiming exact API details.

## Done when

- confirmed at height 100 with tip 100 has 1 confirmation
- confirmed at height 100 with tip 105 has 6 confirmations
- mempool UTXO with no seen height has 0 confirmations
- Required tests pass with `cargo test`.
- The learner can explain the concept in one or two sentences.
