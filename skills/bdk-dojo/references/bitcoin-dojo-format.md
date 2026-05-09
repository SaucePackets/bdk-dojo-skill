# Bitcoin Dojo Format for BDK Dojo

## Source model

BDK Dojo should follow the shape of the public `SaucePackets/bitcoin-dojo` repo:

```text
bitcoin-dojo/
  bitcoin_dojo/
    Cargo.toml
    src/
      lib.rs
      ecc/
      utils/
      transaction/
    tests/
      tests.rs
      ecc/
      utils/
      transaction/
  scaffold/
    1.1-hashing-randomness/
      README.md
      stubs.rs
    2.8-varints/
      README.md
      stubs.rs
    3.6-tx-hash-id/
      README.md
      stubs.rs
```

Important pattern:

- one cumulative Rust crate
- scaffold directory per authored exercise
- no central scaffold index file; the directories are the index
- each exercise has `README.md` instructions
- each exercise has `stubs.rs` starter code
- tests live in the crate and prove the implementation
- modules grow by domain, not by random lesson files
- exercises are numbered so the path is visible
- authored scaffolds remain in the repo after completion
- future scaffolds may be authored ahead of use, but only as complete real exercises: README, stubs, tests, reference implementation, and expected behavior
- never create placeholder scaffold directories

## BDK Dojo target shape

Use this public repo shape for BDK Dojo curriculum assets:

```text
bdk-dojo-skill/
  README.md
  examples/
    bdk-dojo-wallet/
      Cargo.toml
      src/
        lib.rs
        amount.rs
        utxo.rs
        balance.rs
        wallet.rs
        sync.rs
        fees.rs
        coin_selection.rs
        tx_plan.rs
        psbt_review.rs
        descriptors.rs
      tests/
        tests.rs
        wallet_flow.rs
        balance/
        sync/
        spending/
        bdk_bridge/
  scaffold/
    1.1-amounts-and-utxos/
      README.md
      stubs.rs
    1.2-total-balance/
      README.md
      stubs.rs
    1.3-balance-buckets/
      README.md
      stubs.rs
    1.4-wallet-state/
      README.md
      stubs.rs
    2.1-confirmation-depth/
      README.md
      stubs.rs
    2.2-spendability-policy/
      README.md
      stubs.rs
    2.3-sync-events/
      README.md
      stubs.rs
    2.4-checkpoints-and-reorgs/
      README.md
      stubs.rs
    2.5-address-index-gap-limit/
      README.md
      stubs.rs
    3.1-fee-rates-and-vbytes/
      README.md
      stubs.rs
    3.2-coin-selection/
      README.md
      stubs.rs
    3.3-dust-and-change-policy/
      README.md
      stubs.rs
    3.4-transaction-proposal/
      README.md
      stubs.rs
    4.1-psbt-review/
      README.md
      stubs.rs
    4.2-error-handling-pass/
      README.md
      stubs.rs
    4.3-full-toy-send-flow/
      README.md
      stubs.rs
    5.1-bdk-project-orientation/
      README.md
      stubs.rs
    5.2-bdk-balance-examples/
      README.md
      stubs.rs
    5.3-descriptor-mental-model/
      README.md
      stubs.rs
    5.4-bdk-wallet-skeleton/
      README.md
      stubs.rs
    5.5-bdk-sync-example/
      README.md
      stubs.rs
    5.6-contribution-drill/
      README.md
      stubs.rs
    5.7-capstone-wallet-flow/
      README.md
      stubs.rs
  skills/
    bdk-dojo/
      SKILL.md
      references/
        bdk-learning-coverage.md
        bitcoin-dojo-format.md
        context-discipline.md
        course-spine.md
        lesson-template.md
        progress-journal.md
        teach-mode.md
```

Learner practice repo shape:

```text
bdk-dojo/
  Cargo.toml
  src/
    lib.rs
    amount.rs
    utxo.rs
    balance.rs
    wallet.rs
    sync.rs
    fees.rs
    coin_selection.rs
    tx_plan.rs
    psbt_review.rs
    descriptors.rs
  tests/
    tests.rs
    wallet_flow.rs
    balance/
    sync/
    spending/
    bdk_bridge/
```

## Exercise README format

Each scaffold README should be short and mechanical:

```markdown
## Setup

- Read the lesson goal.
- Create or update: `src/<module>.rs`.
- Update `src/lib.rs` to expose the module.
- Copy the stubs from `scaffold/<exercise>/stubs.rs`.
- Write your solution.
- Run: `cargo test`.

## Goal

One Bitcoin/BDK concept in plain language.

## Builds on

- Previous module/function.

## Done when

- Named tests pass.
- One edge case passes.
- No duplicated prior logic.
```

## Stub format

Stubs should show the exact destination path and compile only after the learner fills the implementation when appropriate:

```rust
/// src/balance.rs

use crate::utxo::Utxo;

pub struct BalanceSummary {
    // fields here
}

pub fn classify_balance(utxos: &[Utxo]) -> BalanceSummary {
    todo!("classify confirmed, pending, and spendable buckets")
}
```

## Curriculum direction

BDK Dojo is not generic Bitcoin Dojo repeated in Rust.

Bitcoin Dojo teaches primitives from scratch:

- ECC
- hashes
- keys
- addresses
- transactions

BDK Dojo should teach wallet engineering and BDK readiness:

- wallet state
- UTXO ownership and spendability
- sync state
- descriptors as wallet policy
- fee estimation
- coin selection
- transaction planning
- PSBT review
- persistence/checkpointing
- safe regtest/signet BDK examples
- contribution habits: tests, docs, tiny patches

Use Bitcoin primitives only when they support wallet understanding. Do not rebuild all of Bitcoin Dojo inside BDK Dojo.

## Anti-duplication rule

Before adding any exercise, check:

- What function/module already exists?
- What new behavior does this exercise add?
- What tests prove the new behavior?
- Does the new name encode a new concept?

If the exercise only renames old logic, delete or rewrite it.
