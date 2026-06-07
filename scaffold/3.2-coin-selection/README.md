# 3.2 Coin Selection

## Setup

- Create `src/errors.rs` before implementing coin selection:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WalletError {
    InsufficientFunds { needed: u64, available: u64 },
}
```

Later lessons extend this enum.

- Update `src/lib.rs` with the needed module exports:
  - `pub mod errors;`
  - `pub mod coin_selection;`
  - `pub use errors::WalletError;`
  - `pub use coin_selection::{CoinSelection, select_coins};`

- Work in the cumulative reference crate: `examples/bdk-dojo-wallet/`, or in your own learner repo/fork. Do not assume the repo is named `bdk-dojo`.
- Create or update: `src/coin_selection.rs`.
- Copy the stubs from `scaffold/3.2-coin-selection/stubs.rs` into your codebase when you reach this lesson.
- Copy the tests from `scaffold/3.2-coin-selection/tests.rs` into your project's `tests/` directory.
- Update the `use` import in `tests.rs` to match your Cargo.toml package name.
- Write your solution.
- Run: `cargo test`.

## Goal

Select enough spendable UTXOs to cover a target amount plus estimated fee.

## Builds on

- FeeRate
- Utxo spendability

## Expected behavior

- selects enough policy-spendable inputs for target plus fee using wallet spendability policy at the current tip
- returns InsufficientFunds when available sats are too low

## Required tests

Defined in `scaffold/3.2-coin-selection/tests.rs`:

- `coin_selection_respects_chain_spendability_policy`
- `coin_selection_picks_enough_spendable_utxos`
- `coin_selection_reports_insufficient_funds`

Copy the test file into your project's `tests/` directory. The tests encode the expected behavior — `cargo test` is the pass/fail gate.

## Reference implementation

- `examples/bdk-dojo-wallet/src/coin_selection.rs`

Only compare to the reference after the learner attempts the lesson.

## BDK bridge

- This toy exercise models one wallet-engineering concept. Real BDK includes descriptors, chain sources, persistence, transaction graph state, and stricter policy boundaries.
- Use BDK docs/examples before claiming exact API details.

## Done when

- selects enough policy-spendable inputs for target plus fee using wallet spendability policy at the current tip
- returns InsufficientFunds when available sats are too low
- Required tests pass with `cargo test`.
- The learner can explain the concept in one or two sentences.
