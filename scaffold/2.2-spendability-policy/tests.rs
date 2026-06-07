// tests/lesson_2_2_spendability_policy.rs
//
// Expected behavior verification for lesson 2.2 — Spendability Policy.
// Copy this file into your project's tests/ directory and run:
//
//   cargo test --test lesson_2_2_spendability_policy
//
// The tests will fail with a `todo!` panic until you implement
// `is_spendable` in src/chain.rs.
//
// Utxo fields available from this lesson onward:
//   outpoint: OutPoint, value: Amount, confirmed: bool, spendable: bool,
//   seen_at_height: Option<u32>, coinbase: bool, locked_until: Option<u32>,
//   owned: bool
//
// Spendability rules (all must pass):
//   1. owned == true                  (not a foreign UTXO)
//   2. locked_until.map_or(true, |l| tip_height >= l)   (timelock expired)
//   3. if coinbase: confirmations >= COINBASE_MATURITY   (maturity rule)

// Update this import to match your Cargo.toml package name.
use your_crate_name::{Amount, COINBASE_MATURITY, OutPoint, Utxo, is_spendable};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Build a fully spendable "normal" UTXO, then callers override fields.
fn normal_utxo(txid: &str) -> Utxo {
    Utxo {
        outpoint: OutPoint {
            txid: txid.to_string(),
            vout: 0,
        },
        value: Amount::from_sats(10_000),
        confirmed: true,
        spendable: true,
        seen_at_height: Some(800_000),
        coinbase: false,
        locked_until: None,
        owned: true,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// Verifies all rejection reasons and the two spendable happy-paths in one test
/// (the required test name from the README).
///
/// Cases exercised:
///   a) owned=false               -> not spendable (foreign UTXO)
///   b) locked_until=Some(900), tip=800 -> not spendable (timelock active)
///   c) coinbase, 50 confs, tip 800_050 -> not spendable (immature)
///   d) coinbase, 101 confs, tip 800_100 -> spendable (COINBASE_MATURITY = 100)
///   e) normal UTXO               -> spendable
#[test]
fn spendability_rejects_immature_coinbase_locked_and_foreign_utxos() {
    // -----------------------------------------------------------------------
    // a) Foreign UTXO: owned = false
    // -----------------------------------------------------------------------
    let foreign = Utxo {
        owned: false,
        ..normal_utxo("foreign")
    };
    assert!(
        !is_spendable(&foreign, 800_000),
        "a UTXO we do not own must never be spendable"
    );

    // -----------------------------------------------------------------------
    // b) Timelocked: locked_until = Some(900), tip = 800
    // -----------------------------------------------------------------------
    let locked = Utxo {
        locked_until: Some(900),
        ..normal_utxo("locked")
    };
    assert!(
        !is_spendable(&locked, 800),
        "a UTXO locked until height 900 must not be spendable at tip 800"
    );

    // -----------------------------------------------------------------------
    // c) Immature coinbase: mined at 800_000, tip = 800_050 (only 51 confs)
    //    COINBASE_MATURITY = 100, so still immature.
    // -----------------------------------------------------------------------
    let immature_coinbase = Utxo {
        coinbase: true,
        seen_at_height: Some(800_000),
        ..normal_utxo("coinbase_immature")
    };
    assert_eq!(COINBASE_MATURITY, 100, "sanity-check: COINBASE_MATURITY must be 100");
    assert!(
        !is_spendable(&immature_coinbase, 800_050),
        "coinbase UTXO with only 51 confirmations must not be spendable (need 100)"
    );

    // -----------------------------------------------------------------------
    // d) Mature coinbase: mined at 800_000, tip = 800_100 (101 confs >= 100)
    // -----------------------------------------------------------------------
    let mature_coinbase = Utxo {
        coinbase: true,
        seen_at_height: Some(800_000),
        ..normal_utxo("coinbase_mature")
    };
    assert!(
        is_spendable(&mature_coinbase, 800_100),
        "coinbase UTXO with 101 confirmations must be spendable (COINBASE_MATURITY = 100)"
    );

    // -----------------------------------------------------------------------
    // e) Normal spendable UTXO: owned, not locked, not coinbase
    // -----------------------------------------------------------------------
    let normal = normal_utxo("normal");
    assert!(
        is_spendable(&normal, 800_000),
        "a normal owned untimelocked non-coinbase UTXO must be spendable"
    );
}
