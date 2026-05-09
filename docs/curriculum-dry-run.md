# BDK Dojo Curriculum Dry Run

## What was tested

The curriculum was tested in staged passes against the cumulative reference crate at `examples/bdk-dojo-wallet`.

Commands run:

```bash
cargo test --test wallet_primitives
cargo test --test chain_sync
cargo test --test spending_flow fee_is_vbytes_times_fee_rate
cargo test --test spending_flow coin_selection
cargo test --test spending_flow dust_change
cargo test --test spending_flow transaction_proposal
cargo test --test spending_flow psbt_review
cargo test --test wallet_flow
cargo test --test bdk_bridge
cargo test --test spending_flow descriptor
cargo test
cargo run
```

## Result

All staged checks passed.

The demo prints:

```text
confirmed: 50000
trusted_pending: 20000
untrusted_pending: 10000
total_spendable: 70000
```

## What the course builds

- Phase 1: wallet primitives and balance categories.
- Phase 2: chain state, confirmation depth, spendability, sync events, reorg rollback, and address index tracking.
- Phase 3: fee rates, coin selection, dust/change policy, and unsigned transaction proposals.
- Phase 4: PSBT-style review discipline, explicit wallet errors, and full toy send flow.
- Phase 5: BDK project orientation, descriptor mental model, safe regtest/signet wallet skeleton, BDK sync mapping, contribution drill, and capstone explanation.

## Known boundary

This is a toy wallet engine, not production wallet software.

It intentionally avoids real private keys, real signing, mainnet funds, production descriptor parsing, and direct BDK dependency until the BDK bridge lessons.

## BDK alignment checked

- `bitcoindevkit/bdk` is the lower-level crate workspace (`bdk_chain`, `bdk_core`, `bdk_esplora`, `bdk_electrum`, `bdk_bitcoind_rpc`, `bdk_file_store`).
- `bitcoindevkit/bdk_wallet` contains the high-level descriptor-based `Wallet` API.
- Real BDK balance includes `immature`, `trusted_pending`, `untrusted_pending`, and `confirmed`.
- Real sync distinguishes initial full scan from later sync updates.
- Contribution workflow expects small atomic commits, tests, docs for public items, safe Rust, and reviewable PRs.

## Learner repo recommendation

Do not reset the learner repo. It correctly completed 1.1–1.3 under the scaffold at the time.

Continue with lesson 1.4 and add compatibility fields only when each later lesson introduces them.
