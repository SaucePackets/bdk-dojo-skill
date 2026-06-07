// tests/lesson_2_4_checkpoints_and_reorgs.rs
//
// Expected behavior verification for lesson 2.4 — Checkpoints and Reorgs.
// Copy this file into your project's tests/ directory and run:
//
//   cargo test --test lesson_2_4_checkpoints_and_reorgs
//
// The tests will fail with a `todo!` panic until you implement
// `WalletState::rollback_to_height` in src/wallet.rs.
//
// WalletState fields available from this lesson onward:
//   utxos: Vec<Utxo>, tip_height: u32, checkpoints: Vec<u32>
//
// rollback_to_height(h) must:
//   - set tip_height = h
//   - remove all checkpoints > h
//   - for each UTXO where seen_at_height > h: set confirmed=false, seen_at_height=None
//   - leave UTXOs confirmed at or below h untouched

// Update this import to match your Cargo.toml package name.
use your_crate_name::{Amount, OutPoint, Utxo, WalletState};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn outpoint(txid: &str) -> OutPoint {
    OutPoint {
        txid: txid.to_string(),
        vout: 0,
    }
}

fn confirmed_utxo(txid: &str, sats: u64, seen_at_height: u32) -> Utxo {
    Utxo {
        outpoint: outpoint(txid),
        value: Amount::from_sats(sats),
        confirmed: true,
        spendable: true,
        seen_at_height: Some(seen_at_height),
        coinbase: false,
        locked_until: None,
        owned: true,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// After rollback_to_height(800_000):
///
///   - tip_height is set to 800_000
///   - checkpoints [799_000, 800_000, 800_001, 800_500] → [799_000, 800_000]
///   - UTXO confirmed at 799_500 (≤ 800_000) stays confirmed
///   - UTXO confirmed at 800_000 (= 800_000) stays confirmed
///   - UTXO confirmed at 800_001 (> 800_000) → confirmed=false, seen_at_height=None
///   - UTXO confirmed at 800_500 (> 800_000) → confirmed=false, seen_at_height=None
#[test]
fn rollback_unconfirms_utxos_above_new_tip() {
    let mut wallet = WalletState::new(800_500);

    // Set up a realistic checkpoint history.
    wallet.checkpoints = vec![799_000, 800_000, 800_001, 800_500];
    wallet.tip_height = 800_500;

    // Four UTXOs at different heights spanning the rollback boundary.
    let op_below = outpoint("below");        // 799_500 — safely below
    let op_at = outpoint("at");              // 800_000 — exactly at the rollback target
    let op_above_1 = outpoint("above_one"); // 800_001 — just above: must be unconfirmed
    let op_above_2 = outpoint("above_two"); // 800_500 — well above: must be unconfirmed

    wallet.utxos.push(confirmed_utxo("below", 10_000, 799_500));
    wallet.utxos.push(confirmed_utxo("at", 20_000, 800_000));
    wallet.utxos.push(confirmed_utxo("above_one", 30_000, 800_001));
    wallet.utxos.push(confirmed_utxo("above_two", 40_000, 800_500));

    // Trigger the rollback.
    wallet.rollback_to_height(800_000);

    // --- Tip height ---
    assert_eq!(
        wallet.tip_height, 800_000,
        "tip_height must be updated to the rollback target"
    );

    // --- Checkpoints ---
    assert!(
        !wallet.checkpoints.contains(&800_001),
        "checkpoint 800_001 must be removed (above rollback target)"
    );
    assert!(
        !wallet.checkpoints.contains(&800_500),
        "checkpoint 800_500 must be removed (above rollback target)"
    );
    assert!(
        wallet.checkpoints.contains(&800_000),
        "checkpoint 800_000 must be retained (at rollback target)"
    );
    assert!(
        wallet.checkpoints.contains(&799_000),
        "checkpoint 799_000 must be retained (below rollback target)"
    );

    // --- UTXO at 799_500: untouched ---
    let u_below = wallet.utxos.iter().find(|u| u.outpoint == op_below).unwrap();
    assert!(
        u_below.confirmed,
        "UTXO confirmed at 799_500 must stay confirmed after rollback to 800_000"
    );
    assert_eq!(
        u_below.seen_at_height,
        Some(799_500),
        "UTXO confirmed at 799_500 must keep its seen_at_height"
    );

    // --- UTXO at 800_000: untouched (boundary is inclusive) ---
    let u_at = wallet.utxos.iter().find(|u| u.outpoint == op_at).unwrap();
    assert!(
        u_at.confirmed,
        "UTXO confirmed exactly at the rollback height (800_000) must stay confirmed"
    );
    assert_eq!(
        u_at.seen_at_height,
        Some(800_000),
        "UTXO at rollback height must keep its seen_at_height"
    );

    // --- UTXO at 800_001: must be unconfirmed ---
    let u_above_1 = wallet.utxos.iter().find(|u| u.outpoint == op_above_1).unwrap();
    assert!(
        !u_above_1.confirmed,
        "UTXO confirmed at 800_001 must be unconfirmed after rollback to 800_000"
    );
    assert_eq!(
        u_above_1.seen_at_height, None,
        "UTXO confirmed at 800_001 must have seen_at_height=None after rollback"
    );

    // --- UTXO at 800_500: must be unconfirmed ---
    let u_above_2 = wallet.utxos.iter().find(|u| u.outpoint == op_above_2).unwrap();
    assert!(
        !u_above_2.confirmed,
        "UTXO confirmed at 800_500 must be unconfirmed after rollback to 800_000"
    );
    assert_eq!(
        u_above_2.seen_at_height, None,
        "UTXO confirmed at 800_500 must have seen_at_height=None after rollback"
    );
}
