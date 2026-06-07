// tests/lesson_1_3_balance_buckets.rs
//
// Expected behavior verification for lesson 1.3 — Balance Buckets.
// Copy this file into your project's tests/ directory and run:
//
//   cargo test --test lesson_1_3_balance_buckets
//
// The tests will fail with a `todo!` panic until you implement
// `classify_balance` in src/balance.rs.
//
// Utxo fields available from this lesson onward:
//   outpoint: OutPoint
//   value:    Amount
//   confirmed: bool
//   spendable: bool

// Update this import to match your Cargo.toml package name.
use your_crate_name::{Amount, BalanceSummary, OutPoint, Utxo, classify_balance};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_utxo(txid: &str, vout: u32, sats: u64, confirmed: bool, spendable: bool) -> Utxo {
    Utxo {
        outpoint: OutPoint {
            txid: txid.to_string(),
            vout,
        },
        value: Amount::from_sats(sats),
        confirmed,
        spendable,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// The four cases from the lesson README:
///
/// | confirmed | spendable | sats   | bucket            |
/// |-----------|-----------|--------|-------------------|
/// | true      | true      | 50 000 | confirmed         |
/// | false     | true      | 20 000 | trusted_pending   |
/// | false     | false     | 10 000 | untrusted_pending |
/// | true      | false     |  5 000 | (ignored)         |
///
/// total_spendable = confirmed + trusted_pending = 70 000
#[test]
fn classify_balance_separates_trust_and_spendability() {
    let utxos = vec![
        // confirmed + spendable -> goes into `confirmed`
        make_utxo("aaaa", 0, 50_000, true, true),
        // unconfirmed + spendable -> goes into `trusted_pending`
        make_utxo("bbbb", 0, 20_000, false, true),
        // unconfirmed + not spendable -> goes into `untrusted_pending`
        make_utxo("cccc", 0, 10_000, false, false),
        // confirmed + not spendable -> ignored in this model
        make_utxo("dddd", 0, 5_000, true, false),
    ];

    let summary = classify_balance(&utxos);

    assert_eq!(
        summary.confirmed, 50_000,
        "confirmed+spendable UTXO should land in confirmed bucket"
    );
    assert_eq!(
        summary.trusted_pending, 20_000,
        "unconfirmed+spendable UTXO should land in trusted_pending bucket"
    );
    assert_eq!(
        summary.untrusted_pending, 10_000,
        "unconfirmed+unspendable UTXO should land in untrusted_pending bucket"
    );
    assert_eq!(
        summary.total_spendable, 70_000,
        "total_spendable = confirmed + trusted_pending = 50_000 + 20_000"
    );
}

/// An empty wallet produces a zeroed BalanceSummary.
#[test]
fn classify_balance_empty_wallet_is_zeroed() {
    let utxos: Vec<Utxo> = vec![];

    let summary = classify_balance(&utxos);

    assert_eq!(
        summary,
        BalanceSummary {
            confirmed: 0,
            trusted_pending: 0,
            untrusted_pending: 0,
            total_spendable: 0,
        },
        "empty UTXO slice should yield all-zero BalanceSummary"
    );
}
