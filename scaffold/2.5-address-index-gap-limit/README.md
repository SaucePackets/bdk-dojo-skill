## Setup

- Merge this field into `src/wallet.rs` on `WalletState`:

```rust
pub addresses: Vec<AddressRecord>,
```

- Update `WalletState::new` so `addresses` starts empty.

- Work in the cumulative reference crate: `examples/bdk-dojo-wallet/`, or in your own learner repo/fork. Do not assume the repo is named `bdk-dojo`.
- Create or update: `src/wallet.rs`.
- Copy the stubs from `scaffold/2.5-address-index-gap-limit/stubs.rs` into your codebase when you reach this lesson.
- Write your solution.
- Run: `cargo test`.

## Goal

Track toy derived addresses and return the next unused address.

## Builds on

- WalletState

## Expected behavior

- first call creates index 0
- repeated call returns same unused address
- after marking used, next call creates index 1

## Required tests

- `next_unused_address_reuses_until_marked_used_then_derives_next`

## Reference implementation

- `examples/bdk-dojo-wallet/src/wallet.rs`

Only compare to the reference after the learner attempts the lesson.

## BDK bridge

- This toy exercise models one wallet-engineering concept. Real BDK includes descriptors, chain sources, persistence, transaction graph state, and stricter policy boundaries.
- Use BDK docs/examples before claiming exact API details.

## Done when

- first call creates index 0
- repeated call returns same unused address
- after marking used, next call creates index 1
- Required tests pass with `cargo test`.
- The learner can explain the concept in one or two sentences.
