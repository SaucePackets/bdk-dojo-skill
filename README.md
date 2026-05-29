# bdk-dojo-skill

BDK Dojo is a Bitcoin/Rust learning skill for Hermes.

It turns wallet concepts into small, testable Rust katas: UTXOs, balances, sync state, reorgs, address indexes, fees, coin selection, transaction proposals, PSBT-style review, wallet errors, descriptors, and Miniscript basics.

This is training code, not a production wallet.

## Install

```bash
curl -fsSL https://raw.githubusercontent.com/SaucePackets/bdk-dojo-skill/main/scripts/install-hermes.sh | bash
```

Then start a new Hermes session and ask:

```text
Use bdk-dojo. Give me the first wallet-balance kata.
```

## What gets installed

The installer installs two Hermes skills:

- `SaucePackets/teach-mode-skill/skills/teach-mode` — the learner-first teaching contract.
- `SaucePackets/bdk-dojo-skill/skills/bdk-dojo` — the Bitcoin/Rust wallet-engineering curriculum.

## Repo structure

```text
bdk-dojo-skill/
├── skills/
│   └── bdk-dojo/
│       ├── SKILL.md
│       └── references/
├── scaffold/
│   ├── 1.1-amounts-and-utxos/
│   ├── ...
│   └── 6.4-real-miniscript-bridge/
├── examples/
│   ├── bdk-dojo-wallet/
│   └── wallet-balance-kata/
├── docs/
│   ├── install-hermes.md
│   └── install-openclaw.md
└── scripts/
    └── install-hermes.sh
```

## Key files

- `skills/bdk-dojo/SKILL.md` — main Hermes skill.
- `skills/bdk-dojo/references/course-spine.md` — lesson sequence.
- `skills/bdk-dojo/references/wallet-balance-utxo-model.md` — first beginner kata.
- `skills/bdk-dojo/references/answer-validation.md` — correctness checks.
- `skills/bdk-dojo/references/bitcoin-dojo-format.md` — scaffold format.
- `scaffold/` — numbered lesson directories with `README.md` and `stubs.rs`.
- `examples/bdk-dojo-wallet/` — cumulative toy wallet-training crate.

## Run the reference crate

```bash
cd examples/bdk-dojo-wallet
cargo test
cargo run
```

## Manual loading

Any agent that can load Markdown can use the repo:

1. Load `teach-mode-skill/skills/teach-mode/SKILL.md`.
2. Load `bdk-dojo-skill/skills/bdk-dojo/SKILL.md`.
3. Load the current lesson from `scaffold/<lesson>/README.md` and `stubs.rs`.

The learner writes first. The agent coaches, hints, reviews, verifies, and only rescues when appropriate.
