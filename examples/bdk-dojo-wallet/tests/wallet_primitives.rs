use bdk_dojo_wallet::{
    Amount, BalanceSummary, OutPoint, Utxo, WalletState, calculate_balance, classify_balance,
};

fn utxo(value: u64, confirmed: bool, spendable: bool) -> Utxo {
    Utxo {
        outpoint: OutPoint {
            txid: "00".repeat(32),
            vout: 0,
        },
        value: Amount::from_sats(value),
        confirmed,
        spendable,
    }
}

#[test]
fn amount_preserves_sats_exactly() {
    let amount = Amount::from_sats(50_000);

    assert_eq!(amount.to_sats(), 50_000);
}

#[test]
fn utxo_stores_outpoint_and_value() {
    let utxo = Utxo {
        outpoint: OutPoint {
            txid: "ab".repeat(32),
            vout: 7,
        },
        value: Amount::from_sats(12_345),
        confirmed: true,
        spendable: true,
    };

    assert_eq!(utxo.outpoint.txid, "ab".repeat(32));
    assert_eq!(utxo.outpoint.vout, 7);
    assert_eq!(utxo.value.to_sats(), 12_345);
}

#[test]
fn calculate_balance_sums_all_utxos() {
    let utxos = vec![utxo(50_000, true, true), utxo(20_000, false, true)];

    assert_eq!(calculate_balance(&utxos), 70_000);
}

#[test]
fn calculate_balance_empty_wallet_is_zero() {
    assert_eq!(calculate_balance(&[]), 0);
}

#[test]
fn classify_balance_separates_trust_and_spendability() {
    let utxos = vec![
        utxo(50_000, true, true),
        utxo(20_000, false, true),
        utxo(10_000, false, false),
        utxo(5_000, true, false),
    ];

    assert_eq!(
        classify_balance(&utxos),
        BalanceSummary {
            confirmed: 50_000,
            trusted_pending: 20_000,
            untrusted_pending: 10_000,
            total_spendable: 70_000,
        }
    );
}

#[test]
fn classify_balance_empty_wallet_is_zeroed() {
    assert_eq!(
        classify_balance(&[]),
        BalanceSummary {
            confirmed: 0,
            trusted_pending: 0,
            untrusted_pending: 0,
            total_spendable: 0,
        }
    );
}

#[test]
fn wallet_balance_delegates_to_classify_balance() {
    let mut wallet = WalletState::new(800_000);
    wallet.utxos = vec![
        utxo(50_000, true, true),
        utxo(20_000, false, true),
        utxo(10_000, false, false),
    ];

    assert_eq!(
        wallet.balance(),
        BalanceSummary {
            confirmed: 50_000,
            trusted_pending: 20_000,
            untrusted_pending: 10_000,
            total_spendable: 70_000,
        }
    );
}

#[test]
fn empty_wallet_balance_is_zeroed() {
    let wallet = WalletState::new(800_000);

    assert_eq!(wallet.tip_height, 800_000);
    assert_eq!(
        wallet.balance(),
        BalanceSummary {
            confirmed: 0,
            trusted_pending: 0,
            untrusted_pending: 0,
            total_spendable: 0,
        }
    );
}
