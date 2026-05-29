# BDK Dojo Full Scaffold Curriculum Implementation Plan

> **For Hermes:** Use subagent-driven-development skill to implement this plan task-by-task.

**Goal:** Build a complete Bitcoin Dojo-style scaffold set for BDK Dojo so learners can follow a concrete path from toy wallet primitives to BDK contribution readiness.

**Architecture:** Keep one cumulative reference crate at `examples/bdk-dojo-wallet/`. Each numbered `scaffold/<lesson>/` directory contains `README.md` and `stubs.rs`. The README states setup, file changes, expected behavior, required tests, reference path, and BDK bridge notes. Stubs show starter code only; learner writes the core logic first.

**Tech Stack:** Rust 2024, cargo tests, BDK docs/examples as source material, toy models before real wallet code, no real keys/funds.

---

## Source material to inspect first

- Bitcoin Dojo-style scaffold layout and README style.
- BDK docs and examples for wallet balance, sync/full scan, descriptors, PSBTs, transactions, and persistence.
- Real wallet patterns only as conceptual references: state boundaries, sync lifecycle, coin selection tradeoffs, review-before-signing discipline.

## Non-negotiable scaffold contract

Every scaffold must include:

- `scaffold/<lesson>/README.md`
- `scaffold/<lesson>/stubs.rs`
- exact files to create/update
- expected behavior
- required normal test
- required edge/error test where relevant
- reference implementation path in `examples/bdk-dojo-wallet/`
- BDK bridge note when the toy concept maps to real BDK

No placeholder scaffolds. If a lesson cannot meet this contract, do not add it yet.

---

## Task 1: Audit the current course spine against authored scaffolds

**Objective:** Produce a gap list from `course-spine.md` to actual `scaffold/*` directories.

**Files:**

- Read: `skills/bdk-dojo/references/course-spine.md`
- Read: `scaffold/*/README.md`
- Create: `docs/curriculum-scaffold-audit.md`

**Steps:**

1. List every lesson in the course spine.
2. Mark scaffold status: authored, missing, stale, or needs reference implementation.
3. Mark reference crate status for each lesson.
4. Commit with `docs: audit bdk dojo scaffold coverage`.

## Task 2: Build Phase 2 scaffold set — chain state and sync

**Objective:** Add concrete scaffolds and reference code for confirmation depth, spendability policy, sync events, checkpoints/reorgs, and address index/gap limit.

**Files:**

- Create: `scaffold/2.1-confirmation-depth/{README.md,stubs.rs}`
- Create: `scaffold/2.2-spendability-policy/{README.md,stubs.rs}`
- Create: `scaffold/2.3-sync-events/{README.md,stubs.rs}`
- Create: `scaffold/2.4-checkpoints-and-reorgs/{README.md,stubs.rs}`
- Create: `scaffold/2.5-address-index-gap-limit/{README.md,stubs.rs}`
- Modify/create reference files under `examples/bdk-dojo-wallet/src/`
- Add tests under `examples/bdk-dojo-wallet/tests/`

**Verification:**
Run `cargo test` in `examples/bdk-dojo-wallet` after each lesson.

## Task 3: Build Phase 3 scaffold set — spending decisions

**Objective:** Add concrete scaffolds and reference code for fee rates, coin selection, dust/change, and transaction proposals.

**Files:**

- Create: `scaffold/3.1-fee-rates-and-vbytes/{README.md,stubs.rs}`
- Create: `scaffold/3.2-coin-selection/{README.md,stubs.rs}`
- Create: `scaffold/3.3-dust-and-change-policy/{README.md,stubs.rs}`
- Create: `scaffold/3.4-transaction-proposal/{README.md,stubs.rs}`
- Modify/create reference modules: `fees.rs`, `coin_selection.rs`, `tx_plan.rs`
- Add normal and edge-case tests.

**Verification:**
Run `cargo test`; include insufficient funds and dust/change edge cases.

## Task 4: Build Phase 4 scaffold set — PSBT and review discipline

**Objective:** Add review-before-signing and error handling lessons.

**Files:**

- Create: `scaffold/4.1-psbt-review/{README.md,stubs.rs}`
- Create: `scaffold/4.2-error-handling-pass/{README.md,stubs.rs}`
- Create: `scaffold/4.3-full-toy-send-flow/{README.md,stubs.rs}`
- Modify/create reference modules: `psbt_review.rs`, error types, `tests/wallet_flow.rs`

**Verification:**
Run `cargo test`; prove safe review rejects unknown/unsafe plans.

## Task 5: Build Phase 5 scaffold set — BDK bridge and contribution readiness

**Objective:** Move from toy wallet model to BDK orientation, examples, descriptors, safe regtest/signet skeleton, sync mapping, and contribution drill.

**Files:**

- Create: `scaffold/5.1-bdk-project-orientation/{README.md,stubs.rs}`
- Create: `scaffold/5.2-bdk-balance-examples/{README.md,stubs.rs}`
- Create: `scaffold/5.3-descriptor-mental-model/{README.md,stubs.rs}`
- Create: `scaffold/5.4-bdk-wallet-skeleton/{README.md,stubs.rs}`
- Create: `scaffold/5.5-bdk-sync-example/{README.md,stubs.rs}`
- Create: `scaffold/5.6-contribution-drill/{README.md,stubs.rs}`
- Create: `scaffold/5.7-capstone-wallet-flow/{README.md,stubs.rs}`

**Verification:**
Do not require real funds. Prefer docs/examples/regtest/signet. Include contribution-readiness checks: focused issue reading, tests/docs/examples first, no drive-by rewrites.

## Task 6: Update skill references after scaffolds are real

**Objective:** Keep the skill honest after implementation.

**Files:**

- Modify: `skills/bdk-dojo/SKILL.md`
- Modify: `skills/bdk-dojo/references/course-spine.md`
- Modify: `skills/bdk-dojo/references/bdk-learning-coverage.md`
- Modify: `README.md`

**Verification:**

- `cargo test` passes in reference crate.
- Every scaffold directory has README/stubs.
- No scaffold is placeholder-only.
- Skill says the scaffold set is complete only after the above is true.
