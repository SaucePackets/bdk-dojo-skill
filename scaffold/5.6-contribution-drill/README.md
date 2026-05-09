## Setup

- Work in the cumulative crate: `examples/bdk-dojo-wallet/` or your learner `bdk-dojo/` repo.
- Create or update: `notes/contribution-drill.md`.
- Use the stubs from `scaffold/5.6-contribution-drill/stubs.rs` into your codebase when you reach this lesson.
- Write the Markdown artifact in `notes/contribution-drill.md`.
- The Rust stub is a checklist/reference helper, not required production wallet code.
- Run: `cargo test` to ensure the existing reference crate still passes.

## Goal

Practice a tiny BDK-adjacent contribution workflow: read, reproduce, test, ask, patch.

## Builds on

- BDK project orientation

## Expected behavior

- read issue/docs first
- reproduce locally
- prefer docs/tests/examples
- avoid drive-by rewrites

## Required tests

- `write a short contribution note before claiming an issue`

## Reference implementation

- `scaffold/5.6-contribution-drill/stubs.rs`

Only compare to the reference after the learner attempts the lesson.

## BDK bridge

- This toy exercise models one wallet-engineering concept. Real BDK includes descriptors, chain sources, persistence, transaction graph state, and stricter policy boundaries.
- Use BDK docs/examples before claiming exact API details.

## Real BDK contribution map

- Read `CONTRIBUTING.md` in both `bitcoindevkit/bdk` and `bitcoindevkit/bdk_wallet`.
- Prefer meaningful docs/tests/examples or small bug fixes over drive-by typo PRs.
- Keep commits atomic and reviewable.
- BDK expects tests for new behavior and documentation for public items.

## Done when

- read issue/docs first
- reproduce locally
- prefer docs/tests/examples
- avoid drive-by rewrites
- Existing reference tests pass with `cargo test`, and the Markdown artifact satisfies the review prompt.
- The learner can explain the concept in one or two sentences.
