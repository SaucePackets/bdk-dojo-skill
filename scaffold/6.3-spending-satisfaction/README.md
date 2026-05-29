# 6.3 Spending Satisfaction

## Setup

- Update `src/lib.rs` with the needed module exports:
  - `pub mod spending_satisfaction;`
  - `pub use spending_satisfaction::{SatisfactionSummary, analyze_satisfaction, can_satisfy_with, describe_satisfaction};`

- Work in the cumulative reference crate: `examples/bdk-dojo-wallet/`, or in your own learner repo/fork. Do not assume the repo is named `bdk-dojo`.
- Create or update: `src/spending_satisfaction.rs`.
- Copy the stubs from `scaffold/6.3-spending-satisfaction/stubs.rs` into your codebase when you reach this lesson.
- Write your solution.
- Run: `cargo test`.

## Goal

Analyze what's needed to *spend* from a policy — what signatures, timelocks, and preimages are required, and which branches are possible.

## Builds on

- Policy AST (6.1)
- Policy composition (6.2)

## Expected behavior

- `analyze_satisfaction(policy)` returns a `SatisfactionSummary` describing:
  - `signatures_required`: minimum distinct signatures to satisfy
  - `timelock_required`: the highest timelock that must pass (if any)
  - `preimage_required`: whether a hash preimage is needed
  - `or_branches_count`: how many alternative satisfaction paths exist
- `can_satisfy_with(policy, available_sigs, current_height, has_preimage)` returns true/false
- `describe_satisfaction(policy)` explains in plain English how to satisfy

## Required tests

- `single_key_requires_one_signature_no_timelock`
- `timelocked_policy_requires_height_and_signature`
- `multisig_2_of_3_requires_two_signatures`
- `or_policy_can_be_satisfied_via_either_branch`
- `can_satisfy_with_checks_all_conditions`

## Reference implementation

- `examples/bdk-dojo-wallet/src/spending_satisfaction.rs`

Only compare to the reference after the learner attempts the lesson.

## BDK bridge

- In real miniscript, satisfaction is a typed concept: every fragment knows how to produce its witness data and what resource limits apply.
- The `miniscript::Miniscript::satisfy()` method produces a witness stack given satisfaction material.
- This toy model skips script-level witness construction and resource accounting.

## Done when

- `analyze_satisfaction` correctly computes requirements for all policy types
- `can_satisfy_with` correctly evaluates whether preconditions are sufficient
- `describe_satisfaction` produces clear English explanations
- Required tests pass with `cargo test`
- The learner can explain the concept in one or two sentences
