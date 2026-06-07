// tests/lesson_1_1_amounts_and_utxos.rs
//
// Expected behavior verification for lesson 1.1.
// Copy this file into your project's tests/ directory.
// Update `use bdk_dojo_wallet::` to match your Cargo.toml package name.

use bdk_dojo_wallet::{Amount, OutPoint, Utxo};

/// The Amount wrapper must preserve sats exactly.
///
/// from_sats(n).to_sats() == n  for any u64.
#[test]
fn amount_preserves_sats_exactly() {
    let amount = Amount::from_sats(50_000);

    assert_eq!(amount.to_sats(), 50_000);
}

/// A UTXO must store its location (OutPoint) and value (Amount) correctly.
#[test]
fn utxo_stores_outpoint_and_value() {
    let outpoint = OutPoint {
        txid: "abc".to_string(),
        vout: 1,
    };
    let utxo = Utxo {
        outpoint,
        value: Amount::from_sats(12_345),
    };

    assert_eq!(utxo.outpoint.txid, "abc");
    assert_eq!(utxo.outpoint.vout, 1);
    assert_eq!(utxo.value.to_sats(), 12_345);
}
