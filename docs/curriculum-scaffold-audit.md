# BDK Dojo Scaffold Coverage Audit

Status: complete scaffold surface for current course spine.

## Authored scaffold directories

- `1.1-amounts-and-utxos` — README + stubs present
- `1.2-total-balance` — README + stubs present
- `1.3-balance-buckets` — README + stubs present
- `1.4-wallet-state` — README + stubs present
- `2.1-confirmation-depth` — README + stubs present
- `2.2-spendability-policy` — README + stubs present
- `2.3-sync-events` — README + stubs present
- `2.4-checkpoints-and-reorgs` — README + stubs present
- `2.5-address-index-gap-limit` — README + stubs present
- `3.1-fee-rates-and-vbytes` — README + stubs present
- `3.2-coin-selection` — README + stubs present
- `3.3-dust-and-change-policy` — README + stubs present
- `3.4-transaction-proposal` — README + stubs present
- `4.1-psbt-review` — README + stubs present
- `4.2-error-handling-pass` — README + stubs present
- `4.3-full-toy-send-flow` — README + stubs present
- `5.1-bdk-project-orientation` — README + stubs present
- `5.2-bdk-balance-examples` — README + stubs present
- `5.3-descriptor-mental-model` — README + stubs present
- `5.4-bdk-wallet-skeleton` — README + stubs present
- `5.5-bdk-sync-example` — README + stubs present
- `5.6-contribution-drill` — README + stubs present
- `5.7-capstone-wallet-flow` — README + stubs present

## Verification

- `cargo test` passes in `examples/bdk-dojo-wallet`.
- No central `scaffold/README.md`; directories are the index, matching Bitcoin Dojo.
- Future changes must keep README/stubs/reference/tests in sync.
