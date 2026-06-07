# 6.1 Miniscript Policy Ast

## Setup

- Update `src/lib.rs` with the needed module exports:
  - `pub mod miniscript_ast;`
  - `pub use miniscript_ast::{Policy, policy_kind, count_keys, is_timelocked, is_multisig};`

- Work in the cumulative reference crate: `examples/bdk-dojo-wallet/`, or in your own learner repo/fork. Do not assume the repo is named `bdk-dojo`.
- Create or update: `src/miniscript_ast.rs`.
- Copy the stubs from `scaffold/6.1-miniscript-policy-ast/stubs.rs` into your codebase when you reach this lesson.
- Copy the tests from `scaffold/6.1-miniscript-policy-ast/tests.rs` into your project's `tests/` directory.
- Update the `use` import in `tests.rs` to match your Cargo.toml package name.
- Write your solution.
- Run: `cargo test`.

## Goal

Model Bitcoin spending conditions as a toy policy AST (Abstract Syntax Tree) — the foundation of Miniscript.

## Builds on

- wallet policy
- descriptor mental model (lesson 5.3)

## Expected behavior

- `policy_kind` classifies a policy by its top-level shape ("single key", "timelocked", "composite", "multisig", etc.)
- `count_keys` counts how many keys appear anywhere in a policy tree
- `is_timelocked` returns true if any branch has an `Older` or `After` node
- `is_multisig` returns true if a `Thresh` has more than one key and no other policy types

## Required tests

Defined in `scaffold/6.1-miniscript-policy-ast/tests.rs`:

- `policy_kind_classifies_single_key_as_single_key`
- `policy_kind_identifies_threshold_as_multisig`
- `count_keys_counts_all_keys_in_nested_policy`
- `timelock_detection_works_on_nested_policy`
- `is_multisig_treats_multi_key_threshold_as_multisig`

Copy the test file into your project's `tests/` directory. The tests encode the expected behavior — `cargo test` is the pass/fail gate.

## Reference implementation

- `examples/bdk-dojo-wallet/src/miniscript_ast.rs`

Only compare to the reference after the learner attempts the lesson.

## BDK bridge

- Miniscript is a real framework for expressing Bitcoin spending conditions as a typed AST. The toy `Policy` enum here models the same idea: conditions compose via `and`, `or`, `thresh`, timelocks, and key checks.
- Real miniscript adds type system constraints (satisfaction cost, malleability, resource limits) that this toy model skips.
- Read more: [`miniscript` crate docs](https://docs.rs/miniscript/latest/miniscript/)

## Done when

- `policy_kind` returns the correct classification for each policy shape
- `count_keys` correctly counts keys in nested `and`/`or`/`thresh` trees
- `is_timelocked` and `is_multisig` return correct booleans
- Required tests pass with `cargo test`
- The learner can explain the concept in one or two sentences
