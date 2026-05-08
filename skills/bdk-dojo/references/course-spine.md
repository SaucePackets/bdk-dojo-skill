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

Start simple in `src/lib.rs` only for the first warm-up. Then split into modules once lessons start building on each other.

## Lesson map

### Phase 1 — Wallet primitives

Goal: understand wallet state before touching BDK APIs.

1. **Amounts and UTXOs**
   - Adds: `Amount`, `Utxo`, `OutPoint` toy structs.
   - Function: `calculate_balance(utxos) -> u64`.
   - Teaches: sats as integers, no floats.
   - Avoids repeating: this is the only plain total-balance kata.

2. **Balance buckets**
   - Builds on: lesson 1 UTXOs.
   - Adds: `BalanceSummary`.
   - Function: `classify_balance(utxos) -> BalanceSummary`.
   - Teaches: confirmed, trusted pending, untrusted pending, spendable.
   - Avoids repeating: do not rename `calculate_balance`; classify trust state instead.

3. **Wallet state**
   - Builds on: balance buckets.
   - Adds: `WalletState { utxos, tip_height }`.
   - Method: `wallet.balance()` delegates to `classify_balance`.
   - Teaches: domain objects and method boundaries.

4. **Confirmation depth**
   - Builds on: `tip_height`.
   - Adds: `seen_at_height: Option<u32>` to UTXOs.
   - Function: `confirmations(utxo, tip_height) -> u32`.
   - Teaches: block height, mempool vs confirmed.

5. **Spendability rules**
   - Builds on: confirmation depth.
   - Adds: immature coinbase / locked / foreign toy states.
   - Function: `is_spendable(utxo, tip_height) -> bool`.
   - Teaches: spendable is policy, not just ownership.

### Phase 2 — Sync mental model

Goal: model how wallet state changes.

6. **Sync update events**
   - Adds: `SyncEvent` enum: found, confirmed, spent, reorged.
   - Method: `wallet.apply(event)`.
   - Teaches: wallet data changes over time.

7. **Checkpoints and reorgs**
   - Adds: toy checkpoint list.
   - Function: `rollback_to_height(height)`.
   - Teaches: why sync cannot be “just fetch once.”

8. **Address index and gap limit**
   - Adds: toy derived address records, no real keys.
   - Function: `next_unused_address()`.
   - Teaches: address tracking without deriving real secrets.

### Phase 3 — Spending decisions

Goal: plan spends before building transactions.

9. **Fee rates and vbytes**
   - Adds: `FeeRate`, `TxSizeEstimate`.
   - Function: `fee(vbytes, sat_per_vb)`.
   - Teaches: fees are size-priced, not amount-priced.

10. **Coin selection: exact-ish target**
    - Builds on: spendable UTXOs and fees.
    - Function: `select_coins(target, fee_rate, utxos)`.
    - Teaches: inputs, change, insufficient funds.

11. **Dust and change policy**
    - Builds on: coin selection.
    - Adds: `DUST_LIMIT` and `ChangeDecision`.
    - Teaches: not every leftover should become change.

12. **Transaction proposal**
    - Adds: `TxPlan { selected, recipient, fee, change }`.
    - Function: `propose_transaction(...) -> Result<TxPlan, WalletError>`.
    - Teaches: planning before signing.

### Phase 4 — PSBT and review discipline

Goal: review what a wallet is about to sign.

13. **PSBT review checklist, toy model**
    - Adds: `PsbtReview` struct.
    - Function: `review_plan(tx_plan, wallet_policy)`.
    - Teaches: outputs, fees, change, unknown recipients.

14. **Error handling pass**
    - Refactors prior `Option`/bool returns into `WalletError`.
    - Teaches: maintainer-friendly API surfaces.

15. **Integration test: full toy send flow**
    - Adds: `tests/wallet_flow.rs`.
    - Flow: sync events -> balance -> select coins -> tx proposal -> review.
    - Teaches: features prove themselves together.

### Phase 5 — BDK bridge

Goal: connect toy concepts to real BDK without pretending the toy is production.

16. **BDK project orientation**
    - Task: inspect BDK docs/examples/contribution guidance before API work.
    - Teaches: how to learn the real project without immediately claiming issues.

17. **Read BDK balance examples**
    - Task: compare toy `BalanceSummary` to BDK balance concepts.
    - Teaches: reading docs/examples before coding.

18. **Descriptor mental model**
    - Adds: toy descriptor parser/classifier, not real key parsing.
    - Teaches: descriptor as wallet policy.

19. **BDK wallet skeleton on regtest/signet**
    - Adds: optional example using BDK crates.
    - Teaches: real library setup, safe network only.

20. **BDK sync example**
    - Connects: toy sync events to BDK sync/full-scan model.
    - Teaches: mapping mental models to APIs.

21. **Contribution drill**
    - Task: docs/test/example improvement in the dojo or a tiny BDK-adjacent example.
    - Teaches: small, reviewable OSS work.

22. **Capstone: explain the wallet flow**
    - Task: learner explains UTXO -> sync -> balance -> coin selection -> tx plan -> PSBT review -> BDK mapping.
    - Teaches: readiness through explanation, not just passing tests.

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

If Jerry's existing `calculate_balance` already returns buckets, treat that as lesson 2 already done and move to `WalletState` next.
