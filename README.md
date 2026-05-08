# bdk-dojo-skill

BDK Dojo is a Bitcoin/Rust learning skill bundle for Hermes and OpenClaw.

It turns Bitcoin wallet concepts into small, testable Rust katas. The goal is practical skill: understand wallet primitives, write clear Rust, build test discipline, and grow toward credible open-source Bitcoin contribution.

## What this includes

- `skills/bdk-dojo/SKILL.md` — the teaching workflow and guardrails.
- `skills/bdk-dojo/references/lesson-template.md` — reusable lesson format.
- `skills/bdk-dojo/references/bitcoin-dojo-format.md` — repo/scaffold format adapted from `SaucePackets/bitcoin-dojo`.
- `skills/bdk-dojo/references/course-spine.md` — cumulative lesson plan that prevents repeated logic.
- `skills/bdk-dojo/references/progress-journal.md` — private learner progress/pain-point journal format.
- `skills/bdk-dojo/references/wallet-balance-utxo-model.md` — first beginner kata.
- `scaffold/` — Bitcoin Dojo-style numbered exercise instructions and stubs.
- `examples/wallet-balance-kata/` — runnable Rust starter kata.
- `docs/install-hermes.md` and `docs/install-openclaw.md` — runtime install notes.

## Who it is for

- Rust beginners who want Bitcoin-shaped practice.
- Bitcoin meetup study groups.
- Developers exploring BDK concepts before touching real wallet code.
- Future contributors building the habits maintainers trust: small changes, tests, clear explanations.

## Learning stance

This is not passive AI tutoring.

The learner writes first. The agent coaches, hints, reviews, verifies, and refactors after the attempt.

## First kata

The curriculum now follows Bitcoin Dojo-style numbered scaffolds.

Start here:

```bash
scaffold/1.1-amounts-and-utxos
scaffold/1.2-total-balance
scaffold/1.3-balance-buckets
```

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
├── LICENSE
├── docs/
│   ├── install-hermes.md
│   └── install-openclaw.md
├── scaffold/
│   ├── 1.1-amounts-and-utxos/
│   │   ├── README.md
│   │   └── stubs.rs
│   ├── 1.2-total-balance/
│   │   ├── README.md
│   │   └── stubs.rs
│   └── 1.3-balance-buckets/
│       ├── README.md
│       └── stubs.rs
├── examples/
│   └── wallet-balance-kata/
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           └── main.rs
└── skills/
    └── bdk-dojo/
        ├── SKILL.md
        └── references/
            ├── bitcoin-dojo-format.md
            ├── course-spine.md
            ├── lesson-template.md
            ├── progress-journal.md
            └── wallet-balance-utxo-model.md
```

## Roadmap

The real roadmap lives in `skills/bdk-dojo/references/course-spine.md` and follows the scaffold/test format described in `skills/bdk-dojo/references/bitcoin-dojo-format.md`.

Short shape:

- Phase 1: wallet primitives — amounts, UTXOs, balances, wallet state, confirmations, spendability.
- Phase 2: sync mental model — sync events, checkpoints, reorgs, address index.
- Phase 3: spending decisions — fee rates, coin selection, dust, transaction proposals.
- Phase 4: PSBT and review discipline — review checklist, errors, full toy send flow.
- Phase 5: BDK bridge — compare to BDK examples, descriptors, safe regtest/signet skeleton, contribution drill.

## Note

This repo is intentionally public-clean. It does not include private learning history, personal critique logs, or live progress ledgers.
