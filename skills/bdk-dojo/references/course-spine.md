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

Example learner repo shape. Use the learner's own GitHub repo/fork or local project name; do not assume the repo is named `bdk-dojo`:

```text
bdk-dojo-learner/
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
- Reference path: `examples/bdk-dojo-wallet/src/chain.rs`.
- Scaffold: `scaffold/2.1-confirmation-depth/`.

#### 2.2 — Spendability rules

- Builds on: confirmation depth.
- Adds: immature coinbase / locked / foreign toy states.
- Function: `is_spendable(utxo, tip_height) -> bool`.
- Teaches: spendable is policy, not just ownership.
- Reference path: `examples/bdk-dojo-wallet/src/chain.rs`.
- Scaffold: `scaffold/2.2-spendability-policy/`.

### Phase 2 — Sync mental model

Goal: model how wallet state changes.

#### 2.3 — Sync update events

- Adds: `SyncEvent` enum: found, confirmed, spent, reorged.
- Method: `wallet.apply(event)`.
- Teaches: wallet data changes over time.
- Reference path: `examples/bdk-dojo-wallet/src/wallet.rs`.
- Scaffold: `scaffold/2.3-sync-events/`.

#### 2.4 — Checkpoints and reorgs

- Adds: toy checkpoint list.
- Function: `rollback_to_height(height)`.
- Teaches: why sync cannot be “just fetch once.”
- Reference path: `examples/bdk-dojo-wallet/src/wallet.rs`.
- Scaffold: `scaffold/2.4-checkpoints-and-reorgs/`.

#### 2.5 — Address index and gap limit

- Adds: toy derived address records, no real keys.
- Function: `next_unused_address()`.
- Teaches: address tracking without deriving real secrets.
- Reference path: `examples/bdk-dojo-wallet/src/wallet.rs`.
- Scaffold: `scaffold/2.5-address-index-gap-limit/`.

### Phase 3 — Spending decisions

Goal: plan spends before building transactions.

#### 3.1 — Fee rates and vbytes

- Adds: `FeeRate`, `TxSizeEstimate`.
- Function: `fee(vbytes, sat_per_vb)`.
- Teaches: fees are size-priced, not amount-priced.
- Reference path: `examples/bdk-dojo-wallet/src/fees.rs`.
- Scaffold: `scaffold/3.1-fee-rates-and-vbytes/`.

#### 3.2 — Coin selection: exact-ish target

- Builds on: spendable UTXOs and fees.
- Function: `select_coins(target, fee_rate, utxos, tip_height)`.
- Teaches: inputs, change, insufficient funds.
- Reference path: `examples/bdk-dojo-wallet/src/coin_selection.rs`.
- Scaffold: `scaffold/3.2-coin-selection/`.

#### 3.3 — Dust and change policy

- Builds on: coin selection.
- Adds: `DUST_LIMIT` and `ChangeDecision`.
- Teaches: not every leftover should become change.
- Reference path: `examples/bdk-dojo-wallet/src/change.rs`.
- Scaffold: `scaffold/3.3-dust-and-change-policy/`.

#### 3.4 — Transaction proposal

- Adds: `TxPlan { selected, recipient, fee, change }`.
- Function: `propose_transaction(..., tip_height) -> Result<TxPlan, WalletError>`.
- Teaches: planning before signing.
- Reference path: `examples/bdk-dojo-wallet/src/tx_plan.rs`.
- Scaffold: `scaffold/3.4-transaction-proposal/`.

### Phase 4 — PSBT and review discipline

Goal: review what a wallet is about to sign.

#### 4.1 — PSBT review checklist, toy model

- Adds: `PsbtReview` struct.
- Function: `review_plan(tx_plan, wallet_policy)`.
- Teaches: outputs, fees, change, unknown recipients.
- Reference path: `examples/bdk-dojo-wallet/src/psbt_review.rs`.
- Scaffold: `scaffold/4.1-psbt-review/`.

#### 4.2 — Error handling pass

- Refactors prior `Option`/bool returns into `WalletError`.
- Teaches: maintainer-friendly API surfaces.
- Reference path: `examples/bdk-dojo-wallet/src/errors.rs`.
- Scaffold: `scaffold/4.2-error-handling-pass/`.

#### 4.3 — Integration test: full toy send flow

- Adds: `tests/wallet_flow.rs`.
- Flow: sync events -> balance -> select coins -> tx proposal -> review.
- Teaches: features prove themselves together.
- Reference path: `examples/bdk-dojo-wallet/tests/wallet_flow.rs`.
- Scaffold: `scaffold/4.3-full-toy-send-flow/`.

### Phase 5 — BDK bridge

Goal: connect toy concepts to real BDK without pretending the toy is production.

#### 5.1 — BDK project orientation

- Task: inspect BDK docs/examples/contribution guidance before API work.
- Teaches: how to learn the real project without immediately claiming issues.
- Reference path: `examples/bdk-dojo-wallet/src/bdk_bridge.rs`.
- Scaffold: `scaffold/5.1-bdk-project-orientation/`.

#### 5.2 — Read BDK balance examples

- Task: compare toy `BalanceSummary` to BDK balance concepts.
- Teaches: reading docs/examples before coding.
- Reference path: `examples/bdk-dojo-wallet/src/bdk_bridge.rs`.
- Scaffold: `scaffold/5.2-bdk-balance-examples/`.

#### 5.3 — Descriptor mental model

- Adds: toy descriptor parser/classifier, not real key parsing.
- Teaches: descriptor as wallet policy.
- Reference path: `examples/bdk-dojo-wallet/src/descriptors.rs`.
- Scaffold: `scaffold/5.3-descriptor-mental-model/`.

#### 5.4 — BDK wallet skeleton on regtest/signet

- Adds: optional example using BDK crates.
- Teaches: real library setup, safe network only.
- Reference path: `scaffold/5.4-bdk-wallet-skeleton/stubs.rs`.
- Scaffold: `scaffold/5.4-bdk-wallet-skeleton/`.

#### 5.5 — BDK sync example

- Connects: toy sync events to BDK sync/full-scan model.
- Teaches: mapping mental models to APIs.
- Reference path: `examples/bdk-dojo-wallet/src/bdk_bridge.rs`.
- Scaffold: `scaffold/5.5-bdk-sync-example/`.

#### 5.6 — Contribution drill

- Task: docs/test/example improvement in the dojo or a tiny BDK-adjacent example.
- Teaches: small, reviewable OSS work.
- Reference path: `scaffold/5.6-contribution-drill/stubs.rs`.
- Scaffold: `scaffold/5.6-contribution-drill/`.

#### 5.7 — Capstone: explain the wallet flow

- Task: learner explains UTXO -> sync -> balance -> coin selection -> tx plan -> PSBT review -> BDK mapping.
- Teaches: readiness through explanation, not just passing tests.
- Reference path: `scaffold/5.7-capstone-wallet-flow/stubs.rs`.
- Scaffold: `scaffold/5.7-capstone-wallet-flow/`.

### Phase 6 — Miniscript Deep

Goal: model Bitcoin spending conditions as a policy AST and bridge to the real `miniscript` crate.

#### 6.1 — Miniscript policy AST

- Adds: `Policy` enum (`Key`, `Older`, `After`, `And`, `Or`, `Thresh`, `HashLock`).
- Functions: `policy_kind`, `count_keys`, `is_timelocked`, `is_multisig`.
- Teaches: spending conditions as composable tree nodes.
- Reference path: `examples/bdk-dojo-wallet/src/miniscript_ast.rs`.
- Scaffold: `scaffold/6.1-miniscript-policy-ast/`.

#### 6.2 — Policy composition

- Builds on: Policy AST.
- Functions: `compose_policy`, `describe_policy`, `extract_timelocks`, `required_signatures`.
- Patterns: multisig with timelock recovery, escrow with arbitrator.
- Teaches: real-world spending policy design.
- Reference path: `examples/bdk-dojo-wallet/src/policy_composition.rs`.
- Scaffold: `scaffold/6.2-policy-composition/`.

#### 6.3 — Spending satisfaction

- Builds on: policy composition.
- Adds: `SatisfactionSummary`, `analyze_satisfaction`, `can_satisfy_with`, `describe_satisfaction`.
- Teaches: what's needed to spend from a given policy.
- Reference path: `examples/bdk-dojo-wallet/src/spending_satisfaction.rs`.
- Scaffold: `scaffold/6.3-spending-satisfaction/`.

#### 6.4 — Real miniscript bridge

- Builds on: all prior miniscript lessons.
- Tasks: read real `miniscript` crate docs, map toy concepts to real types, write bridge note.
- Teaches: connecting toy models to production library APIs.
- Reference path: `examples/bdk-dojo-wallet/src/bdk_bridge.rs` (extended).
- Scaffold: `scaffold/6.4-real-miniscript-bridge/`.

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
