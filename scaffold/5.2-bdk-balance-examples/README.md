# 5.2 BDK Balance Examples

## Setup

- Work in the cumulative reference crate: `examples/bdk-dojo-wallet/`, or in your own learner repo/fork. Do not assume the repo is named `bdk-dojo`.
- Create or update: `src/bdk_bridge.rs`.
- Copy the stubs from `scaffold/5.2-bdk-balance-examples/stubs.rs` into your codebase when you reach this lesson.
- Write your solution.
- Run: `cargo test`.

## Goal

Compare toy BalanceSummary to real BDK wallet balance ideas.

## Builds on

- BalanceSummary

## Expected behavior

- name what toy balance hides
- explain why BDK balance depends on indexed chain data

## Required tests

- `bdk_bridge_notes_name_what_the_toy_model_hides`

## Reference implementation

- `examples/bdk-dojo-wallet/src/bdk_bridge.rs`

Only compare to the reference after the learner attempts the lesson.

## BDK bridge

- This toy exercise models one wallet-engineering concept. Real BDK includes descriptors, chain sources, persistence, transaction graph state, and stricter policy boundaries.
- Use BDK docs/examples before claiming exact API details.

## Real BDK balance map

- `bdk_chain::Balance` has `immature`, `trusted_pending`, `untrusted_pending`, and `confirmed` fields.
- It exposes `trusted_spendable()` and `total()` helpers.
- The toy `BalanceSummary` deliberately omits `immature` until spendability policy is introduced.
- Read `crates/chain/src/balance.rs` in `bitcoindevkit/bdk` when comparing models.

## BDK source stamp

Checked against public BDK repository layout and docs on 2026-05-12. Treat exact crate paths, example names, and APIs as version-sensitive; re-check upstream `bitcoindevkit/bdk` and `bitcoindevkit/bdk_wallet` before teaching exact API details.

## Done when

- name what toy balance hides
- explain why BDK balance depends on indexed chain data
- Required tests pass with `cargo test`.
- The learner can explain the concept in one or two sentences.
