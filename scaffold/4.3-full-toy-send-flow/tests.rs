// tests.rs — Lesson 4.3: Full Toy Send Flow (integration test)
//
// This file contains a single end-to-end integration test that wires together
// lessons 3.1 → 4.2:
//
//   WalletState → SyncEvent → balance() → propose_transaction() → review_plan()
//
// Run it with:
//
//   cargo test

// Update this import to match your Cargo.toml package name.
use your_crate_name::{
    propose_transaction, review_plan, Amount, FeeRate, OutPoint, SyncEvent, Utxo, WalletPolicy,
    WalletState,
};

#[cfg(test)]
mod tests {
    use super::*;

    fn make_utxo(value: u64, vout: u32) -> Utxo {
        Utxo {
            outpoint: OutPoint {
                txid: format!("{:064x}", vout),
                vout,
            },
            value: Amount::from_sats(value),
            confirmed: false,
            spendable: true,
            seen_at_height: None,
            coinbase: false,
            locked_until: None,
            owned: true,
        }
    }

    #[test]
    fn full_toy_send_flow_sync_balance_select_propose_review() {
        let mut wallet = WalletState::new(800_000);
        let utxo = make_utxo(100_000, 0);
        let outpoint = utxo.outpoint.clone();

        wallet.apply(SyncEvent::Found(utxo));
        wallet.apply(SyncEvent::Confirmed {
            outpoint,
            height: 799_990,
        });

        let balance = wallet.balance();
        assert_eq!(balance.confirmed, 100_000);
        assert_eq!(balance.total_spendable, 100_000);

        let plan = propose_transaction(
            "bcrt1recipient".to_string(),
            50_000,
            FeeRate { sat_per_vb: 1 },
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
        assert!(review.warnings.is_empty());
    }
}
