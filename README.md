# bdk-dojo-skill

BDK Dojo is a Bitcoin/Rust learning skill bundle for Hermes and OpenClaw.

It turns Bitcoin wallet concepts into small, testable Rust katas. The goal is practical skill: understand wallet primitives, write clear Rust, build test discipline, and grow toward credible open-source Bitcoin contribution.

## What this includes

- `skills/bdk-dojo/SKILL.md` — the teaching workflow and guardrails.
- `skills/bdk-dojo/references/lesson-template.md` — reusable lesson format.
- `skills/bdk-dojo/references/wallet-balance-utxo-model.md` — first beginner kata.
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

Wallet balance from UTXOs:

```bash
cd examples/wallet-balance-kata
cargo test
cargo run
```

The kata models confirmed, trusted pending, untrusted pending, and total spendable balance buckets.

## Repo layout

```text
bdk-dojo-skill/
├── README.md
├── LICENSE
├── docs/
│   ├── install-hermes.md
│   └── install-openclaw.md
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
            ├── lesson-template.md
            └── wallet-balance-utxo-model.md
```

## Roadmap

- Add descriptor parsing katas.
- Add PSBT review/building katas.
- Add wallet sync mental-model lessons.
- Add fee-rate and coin-selection exercises.
- Add BDK API integration lessons.
- Add maintainer-ready contribution drills.

## Note

This repo is intentionally public-clean. It does not include private learning history, personal critique logs, or live progress ledgers.
