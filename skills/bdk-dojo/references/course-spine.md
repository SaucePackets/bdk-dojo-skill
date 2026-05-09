# BDK Dojo Course Spine

## North star

Build a small, tested Rust wallet training repo that grows from toy wallet primitives into BDK-shaped concepts.

The course should avoid repeated logic. Each lesson either adds a new capability, refactors a prior capability into a clearer model, or connects the toy model to real BDK APIs.

The course is learning-first, not issue-claim-first. BDK exploration should move from toy model -> docs/examples comparison -> small local BDK example -> contribution drill.

Final target: a safe learning wallet engine that can model UTXOs, balances, sync state, fee planning, coin selection, transaction proposals, PSBT review, persistence, and then compare those pieces to BDK examples. No real funds. No production keys.

## Teaching loop

Each lesson follows the Prompt Dojo-style loop, adapted for code:

- teach one concept
- show the repo state it builds on
- show one tiny example
- learner writes first
- review correctness, Rust clarity, Bitcoin model, and tests
- ask for one improvement or refactor
- only then show polished code if needed
- map the toy concept to BDK docs/examples when the concept is stable
- record progress, pain points, and still-fuzzy questions

## Repo growth model

Follow the Bitcoin Dojo-style structure described in `bitcoin-dojo-format.md`:

- one cumulative Rust crate
- numbered `scaffold/<exercise>/README.md`
- numbered `scaffold/<exercise>/stubs.rs`
- tests live in the crate and prove each exercise
- modules grow by wallet domain

Default learner repo:

```text
bdk-dojo/
  Cargo.toml
  src/
    lib.rs
    amount.rs
    utxo.rs
    balance.rs
    wallet.rs
    sync.rs
    fees.rs
    coin_selection.rs
    tx_plan.rs
    psbt_review.rs
    descriptors.rs
  tests/
    tests.rs
    wallet_flow.rs
    balance/
    sync/
    spending/
    bdk_bridge/
```

Start simple in `src/lib.rs` only for the first warm-up. Split into modules as soon as lesson 1.1 introduces `amount.rs` and `utxo.rs`.

## Lesson map

### Phase 1 — Wallet primitives

Goal: understand wallet state before touching BDK APIs.

#### 1.1 — Amounts and UTXOs

- Adds: `Amount`, `Utxo`, `OutPoint` toy structs.
- Teaches: sats as integers, no floats; UTXOs as wallet coins.
- Reference path: `examples/bdk-dojo-wallet/src/amount.rs`, `examples/bdk-dojo-wallet/src/utxo.rs`.
- Scaffold: `scaffold/1.1-amounts-and-utxos/`.

#### 1.2 — Total balance

- Builds on: lesson 1.1 UTXOs.
- Function: `calculate_balance(utxos) -> u64`.
- Teaches: summing sats across all wallet UTXOs.
- Avoids repeating: this is the only plain total-balance kata.
- Reference path: `examples/bdk-dojo-wallet/src/balance.rs`.
- Scaffold: `scaffold/1.2-total-balance/`.

#### 1.3 — Balance buckets

- Builds on: lesson 1.2 total balance.
- Adds: `BalanceSummary`.
- Function: `classify_balance(utxos) -> BalanceSummary`.
- Teaches: confirmed, trusted pending, untrusted pending, spendable.
- Avoids repeating: do not rename `calculate_balance`; classify trust state instead.
- Reference path: `examples/bdk-dojo-wallet/src/balance.rs`.
- Scaffold: `scaffold/1.3-balance-buckets/`.

#### 1.4 — Wallet state

- Builds on: balance buckets.
- Adds: `WalletState { utxos, tip_height }`.
- Method: `wallet.balance()` delegates to `classify_balance`.
- Teaches: domain objects and method boundaries.
- Reference path: `examples/bdk-dojo-wallet/src/wallet.rs`.
- Scaffold: `scaffold/1.4-wallet-state/`.

#### 2.1 — Confirmation depth

- Builds on: `tip_height`.
- Adds: `seen_at_height: Option<u32>` to UTXOs.
- Function: `confirmations(utxo, tip_height) -> u32`.
- Teaches: block height, mempool vs confirmed.
- Status: planned, not scaffolded yet.

#### 2.2 — Spendability rules

- Builds on: confirmation depth.
- Adds: immature coinbase / locked / foreign toy states.
- Function: `is_spendable(utxo, tip_height) -> bool`.
- Teaches: spendable is policy, not just ownership.
- Status: planned, not scaffolded yet.

### Phase 2 — Sync mental model

Goal: model how wallet state changes.

#### 2.3 — Sync update events

- Adds: `SyncEvent` enum: found, confirmed, spent, reorged.
- Method: `wallet.apply(event)`.
- Teaches: wallet data changes over time.
- Status: planned, not scaffolded yet.

#### 2.4 — Checkpoints and reorgs

- Adds: toy checkpoint list.
- Function: `rollback_to_height(height)`.
- Teaches: why sync cannot be “just fetch once.”
- Status: planned, not scaffolded yet.

#### 2.5 — Address index and gap limit

- Adds: toy derived address records, no real keys.
- Function: `next_unused_address()`.
- Teaches: address tracking without deriving real secrets.
- Status: planned, not scaffolded yet.

### Phase 3 — Spending decisions

Goal: plan spends before building transactions.

#### 3.1 — Fee rates and vbytes

- Adds: `FeeRate`, `TxSizeEstimate`.
- Function: `fee(vbytes, sat_per_vb)`.
- Teaches: fees are size-priced, not amount-priced.
- Status: planned, not scaffolded yet.

#### 3.2 — Coin selection: exact-ish target

- Builds on: spendable UTXOs and fees.
- Function: `select_coins(target, fee_rate, utxos)`.
- Teaches: inputs, change, insufficient funds.
- Status: planned, not scaffolded yet.

#### 3.3 — Dust and change policy

- Builds on: coin selection.
- Adds: `DUST_LIMIT` and `ChangeDecision`.
- Teaches: not every leftover should become change.
- Status: planned, not scaffolded yet.

#### 3.4 — Transaction proposal

- Adds: `TxPlan { selected, recipient, fee, change }`.
- Function: `propose_transaction(...) -> Result<TxPlan, WalletError>`.
- Teaches: planning before signing.
- Status: planned, not scaffolded yet.

### Phase 4 — PSBT and review discipline

Goal: review what a wallet is about to sign.

#### 4.1 — PSBT review checklist, toy model

- Adds: `PsbtReview` struct.
- Function: `review_plan(tx_plan, wallet_policy)`.
- Teaches: outputs, fees, change, unknown recipients.
- Status: planned, not scaffolded yet.

#### 4.2 — Error handling pass

- Refactors prior `Option`/bool returns into `WalletError`.
- Teaches: maintainer-friendly API surfaces.
- Status: planned, not scaffolded yet.

#### 4.3 — Integration test: full toy send flow

- Adds: `tests/wallet_flow.rs`.
- Flow: sync events -> balance -> select coins -> tx proposal -> review.
- Teaches: features prove themselves together.
- Status: planned, not scaffolded yet.

### Phase 5 — BDK bridge

Goal: connect toy concepts to real BDK without pretending the toy is production.

#### 5.1 — BDK project orientation

- Task: inspect BDK docs/examples/contribution guidance before API work.
- Teaches: how to learn the real project without immediately claiming issues.
- Status: planned, not scaffolded yet.

#### 5.2 — Read BDK balance examples

- Task: compare toy `BalanceSummary` to BDK balance concepts.
- Teaches: reading docs/examples before coding.
- Status: planned, not scaffolded yet.

#### 5.3 — Descriptor mental model

- Adds: toy descriptor parser/classifier, not real key parsing.
- Teaches: descriptor as wallet policy.
- Status: planned, not scaffolded yet.

#### 5.4 — BDK wallet skeleton on regtest/signet

- Adds: optional example using BDK crates.
- Teaches: real library setup, safe network only.
- Status: planned, not scaffolded yet.

#### 5.5 — BDK sync example

- Connects: toy sync events to BDK sync/full-scan model.
- Teaches: mapping mental models to APIs.
- Status: planned, not scaffolded yet.

#### 5.6 — Contribution drill

- Task: docs/test/example improvement in the dojo or a tiny BDK-adjacent example.
- Teaches: small, reviewable OSS work.
- Status: planned, not scaffolded yet.

#### 5.7 — Capstone: explain the wallet flow

- Task: learner explains UTXO -> sync -> balance -> coin selection -> tx plan -> PSBT review -> BDK mapping.
- Teaches: readiness through explanation, not just passing tests.
- Status: planned, not scaffolded yet.

## Naming rules

Use names that show new meaning:

- `calculate_balance`: plain total sats only.
- `classify_balance`: balance buckets and trust state.
- `WalletState::balance`: domain method that calls classification.
- `select_coins`: chooses inputs for a spend.
- `propose_transaction`: creates an unsigned plan.
- `review_plan`: checks whether the plan is sane.

If two lessons produce the same function with different names, the lesson is wrong. Refactor the plan before teaching it.

## Stop signs

Do not start a new lesson until the current one has:

- passing tests
- one edge-case test
- no unexplained warnings
- a clear note about what changed in the wallet model
- no duplicated logic from prior lessons

## Current correction

The earlier `summarize_balance` idea overlaps too much with a prior `calculate_balance` lesson if `calculate_balance` already handles buckets. The intended split is:

- `calculate_balance`: total only
- `classify_balance`: trust/spendability buckets

If the learner's existing `calculate_balance` already returns buckets, treat that as lesson 1.3 already done and move to `WalletState` next.
