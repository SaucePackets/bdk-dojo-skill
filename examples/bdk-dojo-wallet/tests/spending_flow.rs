use bdk_dojo_wallet::{
    Amount, ChangeDecision, DUST_LIMIT, DescriptorKind, FeeRate, OutPoint, TxSizeEstimate, Utxo,
    WalletError, WalletPolicy, classify_descriptor, decide_change, fee, propose_transaction,
    review_plan, select_coins, validate_toy_descriptor,
};

fn outpoint(n: u32) -> OutPoint {
    OutPoint {
        txid: format!("{n:064x}"),
        vout: n,
    }
}

fn utxo(value: u64, n: u32) -> Utxo {
    Utxo::new(outpoint(n), Amount::from_sats(value)).confirmed_at(800_000)
}

#[test]
fn fee_is_vbytes_times_fee_rate() {
    assert_eq!(
        fee(TxSizeEstimate::new(141), FeeRate::from_sat_per_vb(2)),
        282
    );
}

#[test]
fn coin_selection_picks_enough_spendable_utxos() {
    let utxos = vec![utxo(30_000, 1), utxo(80_000, 2)];
    let selection = select_coins(50_000, FeeRate::from_sat_per_vb(1), &utxos, 800_000).unwrap();

    assert!(selection.total_selected >= selection.target + selection.estimated_fee);
    assert_eq!(selection.selected.len(), 2);
}

#[test]
fn coin_selection_respects_chain_spendability_policy() {
    let utxos = vec![utxo(100_000, 1).locked_until(900_000), utxo(75_000, 2)];
    let selection = select_coins(50_000, FeeRate::from_sat_per_vb(1), &utxos, 800_000).unwrap();

    assert_eq!(selection.selected.len(), 1);
    assert_eq!(selection.selected[0].outpoint.vout, 2);
}

#[test]
fn coin_selection_reports_insufficient_funds() {
    let utxos = vec![utxo(10_000, 1)];
    let error = select_coins(50_000, FeeRate::from_sat_per_vb(1), &utxos, 800_000).unwrap_err();

    assert!(matches!(error, WalletError::InsufficientFunds { .. }));
}

#[test]
fn dust_change_is_added_to_fee_instead_of_output() {
    assert_eq!(decide_change(0), ChangeDecision::NoChange);
    assert_eq!(
        decide_change(DUST_LIMIT - 1),
        ChangeDecision::AddToFee(DUST_LIMIT - 1)
    );
    assert_eq!(
        decide_change(DUST_LIMIT),
        ChangeDecision::Change(DUST_LIMIT)
    );
}

#[test]
fn transaction_proposal_contains_selected_inputs_fee_and_change() {
    let utxos = vec![utxo(100_000, 1)];
    let plan = propose_transaction(
        "bcrt1recipient".to_string(),
        50_000,
        FeeRate::from_sat_per_vb(1),
        &utxos,
        800_000,
    )
    .unwrap();

    assert_eq!(plan.recipient, "bcrt1recipient");
    assert_eq!(plan.amount, 50_000);
    assert_eq!(plan.selected.len(), 1);
    assert!(plan.fee > 0);
}

#[test]
fn psbt_review_rejects_unknown_recipient_and_high_fee() {
    let utxos = vec![utxo(100_000, 1)];
    let plan = propose_transaction(
        "bcrt1recipient".to_string(),
        50_000,
        FeeRate::from_sat_per_vb(1),
        &utxos,
        800_000,
    )
    .unwrap();

    let unknown_policy = WalletPolicy {
        allowed_recipients: vec!["bcrt1other".to_string()],
        max_fee: 10_000,
    };
    assert!(matches!(
        review_plan(&plan, &unknown_policy),
        Err(WalletError::UnknownRecipient(_))
    ));

    let high_fee_policy = WalletPolicy {
        allowed_recipients: vec!["bcrt1recipient".to_string()],
        max_fee: 1,
    };
    assert!(matches!(
        review_plan(&plan, &high_fee_policy),
        Err(WalletError::UnsafePsbt(_))
    ));
}

#[test]
fn descriptor_classifier_recognizes_common_policy_shapes() {
    assert_eq!(
        classify_descriptor("wpkh(xpub...)"),
        DescriptorKind::SingleSig
    );
    assert_eq!(
        classify_descriptor("wsh(sortedmulti(2,xpub1,xpub2))"),
        DescriptorKind::MultiSig
    );
    assert_eq!(
        validate_toy_descriptor("not-a-descriptor"),
        Err(WalletError::InvalidDescriptor)
    );
}
