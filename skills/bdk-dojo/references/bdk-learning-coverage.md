# BDK Learning Coverage

## Purpose

BDK Dojo should maximize learning, not just finish exercises.

The curriculum should build four things at once:

- Rust comfort
- Bitcoin wallet mental models
- BDK API understanding
- open-source contribution readiness

Do not rush into claiming BDK issues. Learn the project first, build small local features, read real examples, then approach contribution with context.

## Learning pillars

### 1. Rust fundamentals through wallet code

Every lesson should reinforce one Rust skill:

- module layout: `src/lib.rs`, domain modules, integration tests
- ownership and borrowing: `&[Utxo]`, `Vec<Utxo>`, cloning only when justified
- structs/enums: domain modeling over loose booleans
- `Result` and custom errors
- traits only when they clarify a boundary
- small functions with targeted tests
- readable names and compiler warnings treated as signals

### 2. Bitcoin wallet model

The learner should be able to explain:

- UTXO vs account balance
- outpoints and ownership
- confirmed vs unconfirmed funds
- spendable vs visible funds
- chain tip, confirmation depth, reorgs
- address derivation/gap limit at a toy level
- fee rate, vbytes, dust, change
- coin selection tradeoffs
- transaction plan vs signed transaction
- PSBT review before signing

### 3. BDK bridge

After each toy concept matures, connect it to BDK:

- find the closest BDK concept, type, or example
- compare toy naming to BDK naming
- note what the toy model hides or simplifies
- read docs/examples before writing BDK code
- use safe networks only: regtest or signet
- never use production seeds, real xprvs, or mainnet funds

Use source links or local file paths in lesson notes when inspecting real BDK docs/examples.

### 4. Testing discipline

Each lesson should include:

- one normal test
- one edge-case test
- integration test when features start interacting
- exact expected values for wallet math
- insufficient-funds and invalid-state cases where relevant
- `cargo test` verification
- warnings reviewed, not ignored

### 5. Contribution readiness

Contribution work starts after the learner can read and test small modules.

Before any BDK issue attempt, the learner should practice:

- reading `CONTRIBUTING`/developer docs
- building the project locally
- running a targeted test
- reproducing an example
- making a tiny docs/test improvement
- writing a clear commit message
- asking one focused maintainer question when needed

Avoid drive-by rewrites. Trust is built with small, reviewable work.

## Required lesson artifacts

A finished lesson should leave behind:

- code in the learner repo
- tests proving the new behavior
- progress journal entry
- short learner explanation of the concept
- known pain point or question, if any
- next lesson selected from the course spine

## BDK-readiness milestones

### Milestone A — Toy wallet foundation

The learner can model:

- amount
- UTXO
- balance buckets
- wallet state
- confirmation depth
- spendability

### Milestone B — Sync and spend planning

The learner can model:

- sync events
- reorg rollback
- fee estimation
- coin selection
- dust/change policy
- unsigned transaction proposal

### Milestone C — Review and safety

The learner can model:

- PSBT-like review
- wallet policy checks
- custom errors
- full toy send flow

### Milestone D — BDK bridge

The learner can:

- map toy concepts to BDK concepts
- run a safe BDK example
- explain descriptors as wallet policy
- compare toy sync to BDK sync/full-scan behavior

### Milestone E — Contribution drill

The learner can:

- inspect the BDK repo locally
- run a targeted test or example
- make a tiny docs/test/example improvement
- explain the change clearly

## Session startup checklist

When teaching a BDK Dojo lesson, the agent should inspect or ask for:

- current learner repo state
- completed lessons/progress journal
- current scaffold README and stubs
- relevant tests
- current error output, if debugging

If the learner repo is available locally, inspect files directly. Do not guess.

## Session ending checklist

End each lesson with:

- verdict: complete or not complete
- tests run and result
- concept learned
- pain point overcome
- next lesson
- suggested journal entry
- optional commit message

## Safety rules

- no real seeds
- no real xprvs
- no mainnet funds
- no signing real transactions
- BDK API examples use toy descriptors, regtest, signet, or docs-only inspection
- clearly label toy code as toy code

## Missing-coverage smell test

The skill is missing something if lessons do not include:

- real tests
- learner explanation
- progress tracking
- BDK comparison
- safety boundaries
- final working toy flow
- contribution practice

If any item is missing, patch the curriculum before continuing.
