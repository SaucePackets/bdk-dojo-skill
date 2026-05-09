use bdk_dojo_wallet::{
    Amount, FeeRate, OutPoint, SyncEvent, Utxo, WalletPolicy, WalletState, propose_transaction,
    review_plan,
};

#[test]
fn full_toy_send_flow_can_be_reviewed_before_signing() {
    let mut wallet = WalletState::new(800_000);
    wallet.apply(SyncEvent::Found(
        Utxo::new(
            OutPoint {
                txid: "aa".repeat(32),
                vout: 0,
            },
            Amount::from_sats(100_000),
        )
        .confirmed_at(799_990),
    ));

    let plan = propose_transaction(
        "bcrt1recipient".to_string(),
        50_000,
        FeeRate::from_sat_per_vb(1),
        &wallet.utxos,
        wallet.tip_height,
    )
    .unwrap();

    let policy = WalletPolicy {
        allowed_recipients: vec!["bcrt1recipient".to_string()],
        max_fee: 10_000,
    };
    let review = review_plan(&plan, &policy).unwrap();

    assert!(review.approved);
    assert_eq!(wallet.balance().confirmed, 100_000);
}
