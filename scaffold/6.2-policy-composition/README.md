## Setup

- Update `src/lib.rs` with the needed module exports:
  - `pub mod policy_composition;`
  - `pub use policy_composition::{compose_policy, describe_policy, extract_timelocks, required_signatures};`

- Work in the cumulative reference crate: `examples/bdk-dojo-wallet/`, or in your own learner repo/fork. Do not assume the repo is named `bdk-dojo`.
- Create or update: `src/policy_composition.rs`.
- Copy the stubs from `scaffold/6.2-policy-composition/stubs.rs` into your codebase when you reach this lesson.
- Write your solution.
- Run: `cargo test`.

## Goal

Compose and analyze real-world spending policies from AST building blocks — multisig with timelock recovery, escrow, collaborative custody.

## Builds on

- Policy AST (lesson 6.1)

## Expected behavior

- `compose_policy("multisig_with_recovery", keys, threshold, recovery_key, timelock)` builds `thresh(1, or(thresh(k, keys...), and(recovery_key, older(timelock))))`
- `compose_policy("escrow", agent, buyer, seller, arbitrator)` builds `thresh(2, or(and(agent, buyer), and(agent, seller), and(buyer, seller), arbitrator))`
- `describe_policy` returns a human-readable summary of what a composed policy allows
- `extract_timelocks` returns all distinct timelock values from a policy tree
- `required_signatures` returns the minimum number of signatures needed to satisfy

## Required tests

- `compose_multisig_with_recovery_produces_correct_tree`
- `describe_policy_returns_human_readable_summary_for_known_patterns`
- `extract_timelocks_finds_all_distinct_values`
- `multisig_2_of_3_requires_2_signatures`

## Reference implementation

- `examples/bdk-dojo-wallet/src/policy_composition.rs`

Only compare to the reference after the learner attempts the lesson.

## BDK bridge

- Real miniscript policies like `and(pk(A),or(99@pk(B),older(1000)))` express the same patterns but with typed satisfaction costs.
- The `miniscript` crate provides real policy parsing, compilation, and satisfaction logic.
- Real descriptor strings wrap miniscript policies: `wsh(thresh(2,pk(xpubA),pk(xpubB),pk(xpubC)))`.

## Done when

- `compose_policy` builds correct ASTs for multisig-with-recovery and escrow
- `describe_policy` produces readable summaries
- `extract_timelocks` and `required_signatures` return correct values
- Required tests pass with `cargo test`
- The learner can explain the concept in one or two sentences
