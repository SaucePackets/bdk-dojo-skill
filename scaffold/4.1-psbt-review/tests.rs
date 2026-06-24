// tests.rs — Lesson 4.1: PSBT Review
//
// This file contains pre-written tests for the `review_plan` stub.
// Run them with:
//
//   cargo test
//
// All tests should FAIL until you implement `review_plan` in `src/psbt_review.rs`.
// Once your implementation is correct every test in this file will pass.

// Update this import to match your Cargo.toml package name.
use your_crate_name::{
    review_plan, Amount, ChangeDecision, OutPoint, TxPlan, Utxo, WalletError, WalletPolicy,
};

#[cfg(test)]
mod tests {
    use super::*;

    fn make_utxo(value: u64) -> Utxo {
        Utxo {
            outpoint: OutPoint {
                txid: "aa".repeat(32),
                vout: 0,
            },
            value: Amount::from_sats(value),
            confirmed: true,
            spendable: true,
            seen_at_height: Some(750_000),
            coinbase: false,
            locked_until: None,
            owned: true,
        }
    }

    fn valid_plan() -> TxPlan {
        TxPlan {
            selected: vec![make_utxo(100_000)],
            recipient: "bcrt1recipient".to_string(),
            amount: 50_000,
            fee: 282,
            change: ChangeDecision::Change(49_718),
        }
    }

    fn policy() -> WalletPolicy {
        WalletPolicy {
            allowed_recipients: vec!["bcrt1recipient".to_string()],
            max_fee: 10_000,
        }
    }

    #[test]
    fn valid_plan_passes_psbt_review() {
        let review = review_plan(&valid_plan(), &policy()).unwrap();

        assert!(review.approved);
        assert!(review.warnings.is_empty());
    }

    #[test]
    fn unknown_recipient_fails_review() {
        let mut plan = valid_plan();
        plan.recipient = "bcrt1attacker".to_string();

        assert_eq!(
            review_plan(&plan, &policy()),
            Err(WalletError::UnknownRecipient("bcrt1attacker".to_string()))
        );
    }

    #[test]
    fn excessive_fee_fails_review() {
        let mut plan = valid_plan();
        plan.fee = 50_000;

        assert!(matches!(
            review_plan(&plan, &policy()),
            Err(WalletError::UnsafePsbt(_))
        ));
    }
}
