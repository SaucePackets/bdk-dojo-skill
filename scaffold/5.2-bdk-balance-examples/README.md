## Setup

- Work in the cumulative crate: `examples/bdk-dojo-wallet/` or your learner `bdk-dojo/` repo.
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

## Done when

- name what toy balance hides
- explain why BDK balance depends on indexed chain data
- Required tests pass with `cargo test`.
- The learner can explain the concept in one or two sentences.
