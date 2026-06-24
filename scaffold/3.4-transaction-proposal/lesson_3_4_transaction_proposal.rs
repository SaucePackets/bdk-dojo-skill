// lesson_3_4_transaction_proposal.rs — Lesson 3.4: Transaction Proposal
//
// This file contains pre-written tests for the `propose_transaction` stub.
// Run them with:
//
//   cargo test
//
// All tests should FAIL until you implement `propose_transaction` in `src/tx_plan.rs`.
// Once your implementation is correct every test in this file will pass.

// Update this import to match your Cargo.toml package name.
use your_crate_name::{
    propose_transaction, Amount, ChangeDecision, FeeRate, OutPoint, TxPlan, Utxo, WalletError,
};

#[cfg(test)]
mod tests {
    use super::*;

    fn make_utxo(value: u64, vout: u32, seen_at_height: Option<u32>) -> Utxo {
        Utxo {
            outpoint: OutPoint {
                txid: format!("{:064x}", vout),
                vout,
            },
            value: Amount::from_sats(value),
            confirmed: seen_at_height.is_some(),
            spendable: true,
            seen_at_height,
            coinbase: false,
            locked_until: None,
            owned: true,
        }
    }

    #[test]
    fn transaction_proposal_produces_valid_plan() {
        let utxos = vec![make_utxo(100_000, 0, Some(750_000))];
        let recipient = "bcrt1recipient".to_string();
        let amount = 50_000_u64;
        let fee_rate = FeeRate { sat_per_vb: 2 };
        let tip_height = 800_000_u32;

        let result = propose_transaction(recipient.clone(), amount, fee_rate, &utxos, tip_height);

        assert!(result.is_ok(), "expected Ok(TxPlan)");

        let plan: TxPlan = result.unwrap();
        assert_eq!(plan.recipient, recipient);
        assert_eq!(plan.amount, amount);
        assert!(plan.fee > 0, "fee must be positive");
        assert!(!plan.selected.is_empty(), "selected must be non-empty");

        let selected_total: u64 = plan.selected.iter().map(|u| u.value.to_sats()).sum();
        let change_amount = match plan.change {
            ChangeDecision::Change(amount) => amount,
            ChangeDecision::NoChange | ChangeDecision::AddToFee(_) => 0,
        };

        assert_eq!(selected_total, plan.amount + plan.fee + change_amount);
    }

    #[test]
    fn transaction_proposal_fails_with_insufficient_funds() {
        let utxos: Vec<Utxo> = vec![];
        let result = propose_transaction(
            "bcrt1recipient".to_string(),
            50_000,
            FeeRate { sat_per_vb: 2 },
            &utxos,
            800_000,
        );

        assert!(matches!(result, Err(WalletError::InsufficientFunds { .. })));
    }
}
