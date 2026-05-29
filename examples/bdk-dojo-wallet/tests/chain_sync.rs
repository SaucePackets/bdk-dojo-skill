use bdk_dojo_wallet::{
    Amount, COINBASE_MATURITY, OutPoint, SyncEvent, Utxo, WalletState, confirmations, is_spendable,
};

fn outpoint(n: u32) -> OutPoint {
    OutPoint {
        txid: format!("{n:064x}"),
        vout: n,
    }
}

fn utxo(value: u64, n: u32) -> Utxo {
    Utxo::new(outpoint(n), Amount::from_sats(value))
}

#[test]
fn confirmations_count_from_seen_height_to_tip() {
    let utxo = utxo(50_000, 0).confirmed_at(100);

    assert_eq!(confirmations(&utxo, 100), 1);
    assert_eq!(confirmations(&utxo, 105), 6);
}

#[test]
fn mempool_utxo_has_zero_confirmations() {
    let utxo = utxo(50_000, 0);

    assert_eq!(confirmations(&utxo, 105), 0);
}

#[test]
fn spendability_rejects_immature_coinbase_locked_and_foreign_utxos() {
    let immature_coinbase = utxo(50_000, 1).confirmed_at(100).coinbase();
    let mature_coinbase = utxo(50_000, 2).confirmed_at(100).coinbase();
    let locked = utxo(50_000, 3).locked_until(200);
    let foreign = utxo(50_000, 4).foreign();

    assert!(!is_spendable(
        &immature_coinbase,
        100 + COINBASE_MATURITY - 2
    ));
    assert!(is_spendable(&mature_coinbase, 100 + COINBASE_MATURITY - 1));
    assert!(!is_spendable(&locked, 199));
    assert!(!is_spendable(&foreign, 250));
}

#[test]
fn wallet_apply_tracks_found_confirmed_spent_and_reorged_utxos() {
    let mut wallet = WalletState::new(100);
    let outpoint = outpoint(9);
    wallet.apply(SyncEvent::Found(utxo(12_345, 9)));
    wallet.apply(SyncEvent::Confirmed {
        outpoint: outpoint.clone(),
        height: 101,
    });

    assert_eq!(wallet.utxos[0].seen_at_height, Some(101));

    wallet.apply(SyncEvent::Reorged {
        outpoint: outpoint.clone(),
    });
    assert!(!wallet.utxos[0].confirmed);

    wallet.apply(SyncEvent::Spent(outpoint));
    assert!(wallet.utxos.is_empty());
}

#[test]
fn rollback_unconfirms_utxos_above_new_tip() {
    let mut wallet = WalletState::new(105);
    wallet.utxos.push(utxo(50_000, 1).confirmed_at(104));
    wallet.apply(SyncEvent::TipAdvanced(106));

    wallet.rollback_to_height(103);

    assert_eq!(wallet.tip_height, 103);
    assert!(!wallet.utxos[0].confirmed);
    assert_eq!(wallet.utxos[0].seen_at_height, None);
}

#[test]
fn next_unused_address_reuses_until_marked_used_then_derives_next() {
    let mut wallet = WalletState::new(800_000);

    let first = wallet.next_unused_address();
    let same = wallet.next_unused_address();
    wallet.mark_address_used(first.index);
    let second = wallet.next_unused_address();

    assert_eq!(first, same);
    assert_eq!(second.index, 1);
    assert_eq!(second.address, "bdk-dojo-address-1");
}
