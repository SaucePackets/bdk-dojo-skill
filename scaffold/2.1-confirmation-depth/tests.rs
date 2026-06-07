// tests/lesson_2_1_confirmation_depth.rs
//
// Expected behavior verification for lesson 2.1 — Confirmation Depth.
// Copy this file into your project's tests/ directory and run:
//
//   cargo test --test lesson_2_1_confirmation_depth
//
// The tests will fail with a `todo!` panic until you implement
// `confirmations` in src/chain.rs.
//
// Utxo fields available from this lesson onward:
//   outpoint: OutPoint, value: Amount, confirmed: bool, spendable: bool,
//   seen_at_height: Option<u32>
//
// Confirmation formula:
//   Some(h) -> tip_height - h + 1   (inclusive: mined *at* tip counts as 1)
//   None    -> 0                     (mempool / not yet seen in a block)

// Update this import to match your Cargo.toml package name.
use your_crate_name::{Amount, OutPoint, Utxo, confirmations};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_utxo(txid: &str, seen_at_height: Option<u32>) -> Utxo {
    Utxo {
        outpoint: OutPoint {
            txid: txid.to_string(),
            vout: 0,
        },
        value: Amount::from_sats(1_000),
        confirmed: seen_at_height.is_some(),
        spendable: true,
        seen_at_height,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// UTXOs mined in a block accumulate confirmations relative to the chain tip.
///
///   seen_at_height = 100, tip = 100  ->  1 confirmation  (mined at tip)
///   seen_at_height = 100, tip = 105  ->  6 confirmations (105 - 100 + 1)
#[test]
fn confirmations_count_from_seen_height_to_tip() {
    let utxo_at_tip = make_utxo("aaaa", Some(100));
    assert_eq!(
        confirmations(&utxo_at_tip, 100),
        1,
        "a UTXO mined at the tip height should have exactly 1 confirmation"
    );

    let utxo_buried = make_utxo("bbbb", Some(100));
    assert_eq!(
        confirmations(&utxo_buried, 105),
        6,
        "105 - 100 + 1 = 6 confirmations"
    );
}

/// A mempool UTXO (seen_at_height = None) has zero confirmations regardless
/// of the current tip height.
#[test]
fn mempool_utxo_has_zero_confirmations() {
    let mempool_utxo = make_utxo("cccc", None);

    assert_eq!(
        confirmations(&mempool_utxo, 800_000),
        0,
        "unconfirmed (mempool) UTXO should always return 0"
    );
}
