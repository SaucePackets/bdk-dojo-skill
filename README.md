# bdk-dojo-skill

BDK Dojo is a Bitcoin/Rust learning skill bundle for Hermes and OpenClaw.

It turns Bitcoin wallet concepts into small, testable Rust katas. The goal is practical skill: understand wallet primitives, write clear Rust, build test discipline, and grow toward credible open-source Bitcoin contribution.

BDK Dojo does **not** build a production wallet. It builds simplified, toy versions of components that BDK supports so learners understand the machinery before using the real library: UTXO tracking, balance categories, sync state, reorg handling, address index tracking, fee math, coin selection, transaction proposals, PSBT-style review, wallet errors, and descriptor mental models.

The cumulative reference crate at `examples/bdk-dojo-wallet/` is a functional training engine. It does not connect to testnet/signet/regtest, validate a real chain, manage keys, sign transactions, broadcast transactions, or protect funds. Real networked wallet work comes later through `bdk_wallet`, Esplora/Electrum/bitcoind backends, and safe regtest or signet examples.

## What this includes

- `skills/bdk-dojo/SKILL.md` — Bitcoin/Rust-specific teaching workflow and guardrails.
- External dependency: install `SaucePackets/teach-mode-skill/skills/teach-mode` alongside BDK Dojo for the umbrella learner-first coaching contract.
- `skills/bdk-dojo/references/lesson-template.md` — reusable lesson format.
- `skills/bdk-dojo/references/answer-validation.md` — correctness checks: expected behavior, tests, reference crate comparison.
- `skills/bdk-dojo/references/context-discipline.md` — compact load order so agents do not overload context.
- `skills/bdk-dojo/references/bdk-teach-mode-supplement.md` — AI teacher behavior: hints, review, verification, progress notes.
- `skills/bdk-dojo/references/bitcoin-dojo-format.md` — repo/scaffold format adapted from the Bitcoin Dojo scaffold pattern.
- `skills/bdk-dojo/references/bdk-learning-coverage.md` — coverage checklist for Rust, wallet concepts, BDK bridge work, testing, and contribution readiness.
- `skills/bdk-dojo/references/course-spine.md` — cumulative lesson plan that prevents repeated logic.
- `skills/bdk-dojo/references/progress-journal.md` — private learner progress/pain-point journal format.
- `skills/bdk-dojo/references/wallet-balance-utxo-model.md` — first beginner kata.
- `scaffold/` — Bitcoin Dojo-style numbered exercise directories; each ready lesson has its own `README.md` and `stubs.rs`.
- `examples/bdk-dojo-wallet/` — cumulative working Rust wallet-training crate.
- `examples/wallet-balance-kata/` — legacy runnable Rust starter kata.
- `docs/install-hermes.md` and `docs/install-openclaw.md` — runtime install notes.

## Install

Stable Hermes install:

```bash
curl -fsSL https://github.com/SaucePackets/bdk-dojo-skill/releases/download/v0.0.2/install-hermes.sh | bash
```

This installs both the standalone `teach-mode` skill and `bdk-dojo`. See `docs/install-hermes.md` for manual and contributor installs.

## Who it is for

- Rust beginners who want Bitcoin-shaped practice.
- Bitcoin meetup study groups.
- Developers exploring BDK concepts before touching real wallet code.
- Future contributors building the habits maintainers trust: small changes, tests, clear explanations.

## Learning stance

This is not passive AI tutoring.

The learner writes first. The agent coaches, hints, reviews, verifies, and refactors after the attempt.

This repo does **not** vendor `teach-mode`. Install the standalone `teach-mode-skill` repo alongside BDK Dojo so every dojo can share one canonical teaching contract instead of carrying copied snapshots.

## Manual agent loading

Hermes gets a one-command installer, but BDK Dojo is not Hermes-only. Any AI agent that can load Markdown context can use the package.

1. Load `teach-mode-skill/skills/teach-mode/SKILL.md`.
2. Load `bdk-dojo-skill/skills/bdk-dojo/SKILL.md`.
3. For a lesson, load the current `scaffold/<lesson>/README.md` and `stubs.rs`.
4. Follow learner-first behavior: learner attempts first, agent coaches, verifies, and only rescues when appropriate.

## First kata

The cumulative working crate starts here:

```bash
cd examples/bdk-dojo-wallet
cargo test
cargo run
```

Work through the numbered scaffold directories:

```bash
find scaffold -maxdepth 1 -mindepth 1 -type d | sort
```

Like the Bitcoin Dojo scaffold pattern, there is no central scaffold index. A lesson is teachable when its directory exists and contains both `README.md` and `stubs.rs`. The scaffold set now covers the planned course spine as real exercises with tests and reference implementations, not placeholders. Scaffolds `1.1` through `5.7` cover the beginner-to-advanced BDK bridge track. Scaffolds `6.1` through `6.4` extend into Miniscript deep-dive content.

The older runnable starter remains available:

```bash
cd examples/wallet-balance-kata
cargo test
cargo run
```

That kata models confirmed, trusted pending, untrusted pending, and total spendable balance buckets.

## Repo layout

```text
bdk-dojo-skill/
├── README.md
├── CHANGELOG.md
├── CONTRIBUTING.md
├── RELEASE_CHECKLIST.md
├── SECURITY.md
├── .github/
├── scripts/
├── docs/
│   ├── install-hermes.md
│   ├── install-openclaw.md
│   └── plans/
├── scaffold/
│   ├── 1.1-amounts-and-utxos/
│   ├── ...
│   └── 5.7-capstone-wallet-flow/
│       ├── README.md
│       └── stubs.rs
│   ├── 6.1-miniscript-policy-ast/
│   │   ├── README.md
│   │   └── stubs.rs
│   ├── 6.2-policy-composition/
│   │   ├── README.md
│   │   └── stubs.rs
│   ├── 6.3-spending-satisfaction/
│   │   ├── README.md
│   │   └── stubs.rs
│   └── 6.4-real-miniscript-bridge/
│       ├── README.md
│       └── stubs.rs
├── examples/
│   ├── bdk-dojo-wallet/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── amount.rs
│   │   │   ├── balance.rs
│   │   │   ├── bdk_bridge.rs
│   │   │   ├── chain.rs
│   │   │   ├── change.rs
│   │   │   ├── coin_selection.rs
│   │   │   ├── descriptors.rs
│   │   │   ├── errors.rs
│   │   │   ├── fees.rs
│   │   │   ├── miniscript_ast.rs
│   │   │   ├── policy_composition.rs
│   │   │   ├── psbt_review.rs
│   │   │   ├── spending_satisfaction.rs
│   │   │   ├── tx_plan.rs
│   │   │   ├── utxo.rs
│   │   │   └── wallet.rs
│   │   └── tests/
│   │       ├── bdk_bridge.rs
│   │       ├── chain_sync.rs
│   │       ├── spending_flow.rs
│   │       ├── wallet_flow.rs
│   │       └── wallet_primitives.rs
│   └── wallet-balance-kata/
├── skills/
│   └── bdk-dojo/
│       ├── SKILL.md
│       └── references/
```

## Roadmap

The real roadmap lives in `skills/bdk-dojo/references/course-spine.md` and follows the scaffold/test format described in `skills/bdk-dojo/references/bitcoin-dojo-format.md`.

Short shape:

- Phase 1: wallet primitives — amounts, UTXOs, balances, wallet state, confirmations, spendability.
- Phase 2: sync mental model — sync events, checkpoints, reorgs, address index.
- Phase 3: spending decisions — fee rates, coin selection, dust, transaction proposals.
- Phase 4: PSBT and review discipline — review checklist, errors, full toy send flow.
- Phase 5: BDK bridge — compare to BDK examples, descriptors, safe regtest/signet skeleton, contribution drill.
- Phase 6: Miniscript Deep — policy AST, policy composition, spending satisfaction, real miniscript bridge.

## Note

This repo is intentionally public-clean. It does not include private learning history, personal critique logs, or live progress ledgers.
