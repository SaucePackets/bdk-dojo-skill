## Setup

- Work in the cumulative crate: `examples/bdk-dojo-wallet/` or your learner `bdk-dojo/` repo.
- Create or update: `src/coin_selection.rs`.
- Copy the stubs from `scaffold/3.2-coin-selection/stubs.rs` into your codebase when you reach this lesson.
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

- `coin_selection_picks_enough_spendable_utxos`
- `coin_selection_reports_insufficient_funds`

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
