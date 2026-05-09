## Setup

- Work in the cumulative reference crate: `examples/bdk-dojo-wallet/`, or in your own learner repo/fork. Do not assume the repo is named `bdk-dojo`.
- Create or update: `notes/bdk-wallet-skeleton.md`.
- Use the stubs from `scaffold/5.4-bdk-wallet-skeleton/stubs.rs` into your codebase when you reach this lesson.
- Write the Markdown artifact in `notes/bdk-wallet-skeleton.md`.
- The Rust stub is a checklist/reference helper, not required production wallet code.
- Run: `cargo test` to ensure the existing reference crate still passes.

## Goal

Sketch a safe regtest/signet-only BDK wallet skeleton without real funds.

## Builds on

- descriptor mental model

## Expected behavior

- uses example descriptors only
- names regtest/signet safety boundary
- does not require real seed phrases or funds

## Required tests

- `read README and keep code as safe pseudocode`

## Reference implementation

- `scaffold/5.4-bdk-wallet-skeleton/stubs.rs`

Only compare to the reference after the learner attempts the lesson.

## BDK bridge

- This toy exercise models one wallet-engineering concept. Real BDK includes descriptors, chain sources, persistence, transaction graph state, and stricter policy boundaries.
- Use BDK docs/examples before claiming exact API details.

## Real BDK wallet map

- `bdk_wallet::Wallet` is descriptor-based and built on `bdk_chain` components.
- Current examples live in `bitcoindevkit/bdk_wallet/examples/`: `esplora_blocking.rs`, `esplora_async.rs`, `electrum.rs`, and `bitcoind_rpc.rs`.
- Wallet state should be persisted with `ChangeSet` storage such as SQLite or `bdk_file_store`.
- Keep this lesson on regtest/signet. No real keys. No mainnet funds.

## Done when

- uses example descriptors only
- names regtest/signet safety boundary
- does not require real seed phrases or funds
- Existing reference tests pass with `cargo test`, and the Markdown artifact satisfies the review prompt.
- The learner can explain the concept in one or two sentences.
