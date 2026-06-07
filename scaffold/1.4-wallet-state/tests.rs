// tests/lesson_1_4_wallet_state.rs
//
// Expected behavior verification for lesson 1.4 — Wallet State.
// Copy this file into your project's tests/ directory and run:
//
//   cargo test --test lesson_1_4_wallet_state
//
// The tests will fail with `todo!` panics until you implement
// `WalletState::new` and `WalletState::balance` in src/wallet.rs.
//
// Utxo fields available from lesson 1.3 onward:
//   outpoint: OutPoint, value: Amount, confirmed: bool, spendable: bool

// Update this import to match your Cargo.toml package name.
use your_crate_name::{Amount, BalanceSummary, OutPoint, Utxo, WalletState};

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

/// WalletState::balance() must delegate to classify_balance and bucket UTXOs
/// identically to lesson 1.3:
///
///   50 000 confirmed+spendable   -> confirmed = 50 000
///   20 000 unconfirmed+spendable -> trusted_pending = 20 000
///   10 000 unconfirmed+unspend.  -> untrusted_pending = 10 000
///   total_spendable              = 70 000
#[test]
fn wallet_balance_delegates_to_classify_balance() {
    let mut wallet = WalletState::new(800_000);

    // Verify the wallet was created at the expected tip height.
    assert_eq!(wallet.tip_height, 800_000);

    // Populate with three UTXOs covering each active bucket.
    wallet.utxos.push(make_utxo("aaaa", 0, 50_000, true, true));
    wallet.utxos.push(make_utxo("bbbb", 0, 20_000, false, true));
    wallet.utxos.push(make_utxo("cccc", 0, 10_000, false, false));

    let summary = wallet.balance();

    assert_eq!(
        summary.confirmed, 50_000,
        "confirmed+spendable UTXO should be in confirmed bucket"
    );
    assert_eq!(
        summary.trusted_pending, 20_000,
        "unconfirmed+spendable UTXO should be in trusted_pending bucket"
    );
    assert_eq!(
        summary.untrusted_pending, 10_000,
        "unconfirmed+unspendable UTXO should be in untrusted_pending bucket"
    );
    assert_eq!(
        summary.total_spendable, 70_000,
        "total_spendable = confirmed + trusted_pending = 70 000"
    );
}

/// A freshly constructed WalletState has no UTXOs; balance() must return zeros.
#[test]
fn empty_wallet_balance_is_zeroed() {
    let wallet = WalletState::new(800_000);

    assert_eq!(
        wallet.balance(),
        BalanceSummary {
            confirmed: 0,
            trusted_pending: 0,
            untrusted_pending: 0,
            total_spendable: 0,
        },
        "a brand-new wallet with no UTXOs should report all-zero balance"
    );
}
