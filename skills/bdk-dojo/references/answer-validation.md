# BDK Dojo Answer Validation

## Purpose

BDK Dojo should not rely on the agent guessing whether an answer is correct.

Each lesson needs an explicit correctness path:

- scaffold instructions define expected behavior
- tests prove behavior mechanically
- the cumulative reference crate shows one working implementation
- the agent reviews the Bitcoin/BDK model separately from compiler success

## Correctness sources

Use these in order:

1. Current scaffold `README.md`
   - files to create or update
   - goal
   - expected behavior
   - done criteria

2. Current scaffold `stubs.rs`
   - required function/type names
   - destination file paths
   - API shape

3. Learner repo tests
   - normal case
   - edge case
   - error case when relevant

4. Public reference crate
   - `examples/bdk-dojo-wallet/`
   - shows one accepted implementation path
   - used by the agent for comparison after the learner attempts the lesson

5. BDK bridge note
   - closest BDK concept/API/example
   - what the toy model hides
   - what to read next

## Agent behavior

The agent should not judge correctness from vibes.

For every completed lesson, verify:

```bash
cargo test
```

When relevant, also verify:

```bash
cargo run
```

Then compare the learner implementation against:

- required API names
- expected outputs
- edge-case tests
- safety boundaries
- BDK bridge note

Also inspect for incomplete implementation markers before giving conceptual feedback:

- `todo!()`
- `unimplemented!()`
- placeholder `panic!()` calls
- placeholder constant returns that ignore required inputs
- unused target-function parameters that should drive behavior

If any are present in the lesson target function, do not talk around it. Say the function is still unimplemented, name the exact function, and explain that downstream test failures are expected until that body exists.

## Answer key policy

The public reference crate is the answer key, but it should not be dumped before the learner tries.

Teach Mode order:

1. learner reads scaffold
2. learner writes first attempt
3. agent gives hints if stuck
4. agent runs/reviews tests
5. only then agent may compare to `examples/bdk-dojo-wallet/`
6. agent explains differences without shaming or over-rewriting

## Lesson authoring requirement

Each new scaffold lesson should include:

```text
Expected behavior:
- exact outputs or state changes

Required tests:
- normal case
- edge case

Reference implementation:
- path in examples/bdk-dojo-wallet once implemented
```

If a lesson lacks expected behavior or tests, do not teach it yet. Patch the lesson first.

## Review checklist

```text
API shape matches scaffold: yes/no
Target function implemented, not placeholder: yes/no
Tests call the lesson target function: yes/no
Normal test passes: yes/no
Edge test passes: yes/no
Test location/scope is appropriate: yes/no
No duplicated prior logic: yes/no
Bitcoin model correct: yes/no
BDK bridge note accurate: yes/no
Reference crate comparison done after attempt: yes/no
```

If tests fail, teach the reason before showing a fix. For Rust beginners, name the concept: module scope, imports, ownership, borrowing, visibility, derives, or crate naming.

If tests pass with warnings, do not call the lesson clean. Name the warning and give the smallest cleanup. A green test suite with unused imports or unused variables is still not ready for completion.

## Important distinction

Passing tests means behavior matched the exercise.

It does not automatically mean the learner understands the model.

The learner still needs to explain the concept in one or two sentences before the lesson is complete.
