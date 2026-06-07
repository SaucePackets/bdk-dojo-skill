// tests/lesson_1_2_total_balance.rs
//
// Expected behavior verification for lesson 1.2 — Total Balance.
// Copy this file into your project's tests/ directory and run:
//
//   cargo test --test lesson_1_2_total_balance
//
// The tests will fail with a `todo!` panic until you implement
// `calculate_balance` in src/balance.rs.

// Update this import to match your Cargo.toml package name.
use your_crate_name::{Amount, OutPoint, Utxo, calculate_balance};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_utxo(txid: &str, vout: u32, sats: u64) -> Utxo {
    Utxo {
        outpoint: OutPoint {
            txid: txid.to_string(),
            vout,
        },
        value: Amount::from_sats(sats),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// An empty wallet has no UTXOs, so the total balance must be exactly zero.
#[test]
fn calculate_balance_empty_wallet_is_zero() {
    let utxos: Vec<Utxo> = vec![];

    let balance = calculate_balance(&utxos);

    assert_eq!(balance, 0, "empty UTXO slice should return 0");
}

/// All UTXOs are summed regardless of any other property.
///
/// Two UTXOs worth 50 000 and 20 000 sats must produce a total of 70 000.
#[test]
fn calculate_balance_sums_all_utxos() {
    let utxos = vec![
        make_utxo("aaaa", 0, 50_000),
        make_utxo("bbbb", 1, 20_000),
    ];

    let balance = calculate_balance(&utxos);

    assert_eq!(balance, 70_000, "50_000 + 20_000 should equal 70_000");
}
