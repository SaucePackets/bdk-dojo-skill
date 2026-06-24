# 5.5 BDK Sync Example

## Setup

- Work in the cumulative reference crate: `examples/bdk-dojo-wallet/`, or in your own learner repo/fork.
- Learner crate convention: package name `bdk-dojo`, Rust import name `bdk_dojo`.
- Create or update: `src/bdk_bridge.rs`.
- Copy the stubs from `scaffold/5.5-bdk-sync-example/stubs.rs` into your codebase when you reach this lesson.
- Copy the tests from `scaffold/5.5-bdk-sync-example/lesson_5_5_bdk_sync_example.rs` into `tests/lesson_5_5_bdk_sync_example.rs`.
- Keep the test import as `bdk_dojo`; the Cargo package should be named `bdk-dojo` (hyphen in package, underscore in Rust import).
- Write your solution.
- Run: `cargo test`.

## Goal

Map toy SyncEvent flow to BDK sync/full-scan concepts.

## Builds on

- SyncEvent
- checkpoints

## Expected behavior

- toy sync is event mutation
- BDK sync updates indexed wallet state and checkpoints

## Required tests

Defined in `scaffold/5.5-bdk-sync-example/lesson_5_5_bdk_sync_example.rs`:

- `sync_bridge_note_names_real_bdk_sync_concepts`

Copy the test file into `tests/lesson_5_5_bdk_sync_example.rs`. The tests encode the expected behavior — `cargo test` is the pass/fail gate.

## Reference implementation

- `examples/bdk-dojo-wallet/src/bdk_bridge.rs`

Only compare to the reference after the learner attempts the lesson.

## BDK bridge

- This toy exercise models one wallet-engineering concept. Real BDK includes descriptors, chain sources, persistence, transaction graph state, and stricter policy boundaries.
- Use BDK docs/examples before claiming exact API details.

## Real BDK sync map

- BDK examples distinguish initial `full_scan` from later `sync` updates.
- Chain sources include Esplora, Electrum, and bitcoind RPC.
- Real sync updates local chain/checkpoint state and indexed transaction graph data; the toy `SyncEvent` model only teaches the lifecycle shape.

## BDK source stamp

Checked against public BDK repository layout and docs on 2026-05-12. Treat exact crate paths, example names, and APIs as version-sensitive; re-check upstream `bitcoindevkit/bdk` and `bitcoindevkit/bdk_wallet` before teaching exact API details.

## Done when

- toy sync is event mutation
- BDK sync updates indexed wallet state and checkpoints
- Required tests pass with `cargo test`.
- The learner can explain the concept in one or two sentences.
