# BDK Dojo Scaffold Index

## Purpose

This directory contains lesson scaffolds that are ready to teach.

Do not confuse the full course spine with implemented scaffolds:

- `skills/bdk-dojo/references/course-spine.md` lists the planned curriculum.
- `scaffold/<lesson>/README.md` + `stubs.rs` means the lesson is authored and teachable.
- `examples/bdk-dojo-wallet/` contains the reference implementation path as lessons become real.

## Ready lessons

These lessons are authored and ready to teach now:

```text
1.1-amounts-and-utxos
1.2-total-balance
1.3-balance-buckets
```

## Planned lessons

These are in the course spine but still need scaffold README/stubs, expected behavior, required tests, and reference implementation before teaching:

```text
1.4-wallet-state
2.1-confirmation-depth
2.2-spendability-policy
2.3-sync-events
2.4-checkpoints-and-reorgs
2.5-address-index-gap-limit
3.1-fee-rates-and-vbytes
3.2-coin-selection
3.3-dust-and-change-policy
3.4-transaction-proposal
4.1-psbt-review
4.2-error-handling-pass
4.3-full-toy-send-flow
5.1-bdk-project-orientation
5.2-bdk-balance-examples
5.3-descriptor-mental-model
5.4-bdk-wallet-skeleton
5.5-bdk-sync-example
5.6-contribution-drill
5.7-capstone-wallet-flow
```

## Rule for adding a lesson

A planned lesson becomes teachable only when it has:

- `scaffold/<lesson>/README.md`
- `scaffold/<lesson>/stubs.rs`
- expected behavior
- required normal and edge-case tests
- reference implementation path in `examples/bdk-dojo-wallet/`
- BDK bridge note when relevant

If those are missing, do not teach that lesson yet. Author the scaffold first.
