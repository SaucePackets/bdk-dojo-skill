// tests/lesson_2_3_sync_events.rs
//
// Expected behavior verification for lesson 2.3 — Sync Events.
// Copy this file into your project's tests/ directory and run:
//
//   cargo test --test lesson_2_3_sync_events
//
// The tests will fail with a `todo!` panic until you implement
// `WalletState::apply` in src/wallet.rs.
//
// SyncEvent variants:
//   Found(Utxo)                         — add UTXO to wallet
//   Confirmed { outpoint, height }      — mark UTXO confirmed at height
//   Spent(OutPoint)                     — remove UTXO from wallet
//   Reorged { outpoint }                — unconfirm UTXO (seen_at_height -> None)
//   TipAdvanced(u32)                    — advance wallet tip_height

// Update this import to match your Cargo.toml package name.
use your_crate_name::{Amount, OutPoint, SyncEvent, Utxo, WalletState};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn outpoint(txid: &str, vout: u32) -> OutPoint {
    OutPoint {
        txid: txid.to_string(),
        vout,
    }
}

fn mempool_utxo(txid: &str, vout: u32, sats: u64) -> Utxo {
    Utxo {
        outpoint: outpoint(txid, vout),
        value: Amount::from_sats(sats),
        confirmed: false,
        spendable: true,
        seen_at_height: None,
        coinbase: false,
        locked_until: None,
        owned: true,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// Exercises all five SyncEvent variants in a realistic sequence:
///
///  1. Found       → UTXO appears in the wallet
///  2. Confirmed   → UTXO gets seen_at_height + confirmed=true
///  3. TipAdvanced → wallet.tip_height advances
///  4. Reorged     → UTXO loses confirmation (confirmed=false, seen_at_height=None)
///  5. Spent       → UTXO is removed from wallet.utxos
#[test]
fn wallet_apply_tracks_found_confirmed_spent_and_reorged_utxos() {
    let mut wallet = WalletState::new(800_000);
    let op_a = outpoint("aaaa", 0);
    let op_b = outpoint("bbbb", 0);

    // ------------------------------------------------------------------
    // 1. Found: both UTXOs arrive unconfirmed from the mempool.
    // ------------------------------------------------------------------
    wallet.apply(SyncEvent::Found(mempool_utxo("aaaa", 0, 50_000)));
    wallet.apply(SyncEvent::Found(mempool_utxo("bbbb", 0, 20_000)));

    assert_eq!(wallet.utxos.len(), 2, "both UTXOs should be present after Found");

    let a = wallet.utxos.iter().find(|u| u.outpoint == op_a).unwrap();
    assert!(!a.confirmed, "newly Found UTXO should not be confirmed yet");
    assert_eq!(a.seen_at_height, None, "newly Found UTXO should have no seen_at_height");

    // ------------------------------------------------------------------
    // 2. Confirmed: UTXO A gets mined in block 800_001.
    // ------------------------------------------------------------------
    wallet.apply(SyncEvent::Confirmed {
        outpoint: op_a.clone(),
        height: 800_001,
    });

    let a = wallet.utxos.iter().find(|u| u.outpoint == op_a).unwrap();
    assert!(a.confirmed, "UTXO A should be confirmed after Confirmed event");
    assert_eq!(
        a.seen_at_height,
        Some(800_001),
        "UTXO A seen_at_height should be the confirmed block height"
    );

    // UTXO B is still unconfirmed.
    let b = wallet.utxos.iter().find(|u| u.outpoint == op_b).unwrap();
    assert!(!b.confirmed, "UTXO B should still be unconfirmed");

    // ------------------------------------------------------------------
    // 3. TipAdvanced: chain advances to 800_002.
    // ------------------------------------------------------------------
    wallet.apply(SyncEvent::TipAdvanced(800_002));
    assert_eq!(
        wallet.tip_height, 800_002,
        "tip_height should update to 800_002 after TipAdvanced"
    );

    // ------------------------------------------------------------------
    // 4. Reorged: block 800_001 is reorganised away; UTXO A reverts.
    // ------------------------------------------------------------------
    wallet.apply(SyncEvent::Reorged {
        outpoint: op_a.clone(),
    });

    let a = wallet.utxos.iter().find(|u| u.outpoint == op_a).unwrap();
    assert!(!a.confirmed, "UTXO A should be unconfirmed after Reorged");
    assert_eq!(
        a.seen_at_height, None,
        "UTXO A seen_at_height should be None after Reorged"
    );

    // Both UTXOs should still be present in the wallet.
    assert_eq!(wallet.utxos.len(), 2, "both UTXOs should still exist after Reorged");

    // ------------------------------------------------------------------
    // 5. Spent: UTXO B is consumed by a transaction.
    // ------------------------------------------------------------------
    wallet.apply(SyncEvent::Spent(op_b.clone()));

    assert_eq!(wallet.utxos.len(), 1, "only one UTXO should remain after Spent");
    assert!(
        wallet.utxos.iter().all(|u| u.outpoint != op_b),
        "spent UTXO B must be removed from wallet.utxos"
    );
}
