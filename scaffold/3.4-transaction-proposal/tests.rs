// tests.rs — Lesson 3.4: Transaction Proposal
//
// This file contains pre-written tests for the `propose_transaction` stub.
// Run them with:
//
//   cargo test
//
// All tests should FAIL until you implement `propose_transaction` in `src/lib.rs`.
// Once your implementation is correct every test in this file will pass.

// Update this import to match your Cargo.toml package name.
use your_crate_name::{propose_transaction, FeeRate, TxPlan, Utxo, WalletError};

#[cfg(test)]
mod tests {
    use super::*;

    // ---------------------------------------------------------------------------
    // Helper: build a simple confirmed Utxo with the given value.
    // Adjust field names to match your actual Utxo struct definition.
    // ---------------------------------------------------------------------------
    fn make_utxo(value: u64, block_height: u32) -> Utxo {
        Utxo {
            value,
            block_height: Some(block_height),
            // Add any additional required fields with sensible dummy values.
        }
    }

    /// propose_transaction must return a TxPlan with:
    ///   - recipient_amount equal to the requested target
    ///   - a positive fee (the network is never free)
    ///   - a non-empty selection whose total is >= target + fee
    ///   - optional change that is above the dust limit (or None)
    ///
    /// Setup:
    ///   One UTXO worth 100_000 sat, target 50_000 sat at 2 sat/vB.
    #[test]
    fn transaction_proposal_produces_valid_plan() {
        let utxos = vec![make_utxo(100_000, 750_000)];
        let target = 50_000_u64;
        let fee_rate = FeeRate { sat_per_vb: 2 };
        let tip_height = 800_000_u32;

        let result = propose_transaction(&utxos, target, fee_rate, tip_height);

        assert!(
            result.is_ok(),
            "expected Ok(TxPlan) but got: {:?}",
            result
        );

        let plan: TxPlan = result.unwrap();

        // The plan must send exactly what was requested.
        assert_eq!(
            plan.recipient_amount, target,
            "recipient_amount must equal the requested target"
        );

        // A valid transaction always pays a non-zero fee.
        assert!(
            plan.fee > 0,
            "fee must be positive, got {} sat",
            plan.fee
        );

        // At least one UTXO must have been selected.
        assert!(
            !plan.selected.is_empty(),
            "selected must be non-empty"
        );

        // The total of selected coins must cover both recipient and fee.
        let selected_total: u64 = plan.selected.iter().map(|u| u.value).sum();
        assert!(
            selected_total >= target + plan.fee,
            "selected total ({selected_total}) must be >= target ({target}) + fee ({})",
            plan.fee
        );

        // If change is present it must be a positive amount
        // (dust filtering should prevent zero-value change outputs).
        if let Some(change) = plan.change {
            assert!(change > 0, "change output must be positive, got {change}");
        }

        // Conservation: selected = recipient + fee + change (or no change).
        let expected_total = target + plan.fee + plan.change.unwrap_or(0);
        assert_eq!(
            selected_total, expected_total,
            "funds must be fully accounted for: selected={selected_total}, \
             recipient={target}+fee={}+change={}",
            plan.fee,
            plan.change.unwrap_or(0)
        );
    }

    /// When no UTXOs are provided the proposal must fail with an appropriate error.
    #[test]
    fn transaction_proposal_fails_with_insufficient_funds() {
        let utxos: Vec<Utxo> = vec![];
        let target = 50_000_u64;
        let fee_rate = FeeRate { sat_per_vb: 2 };
        let tip_height = 800_000_u32;

        let result = propose_transaction(&utxos, target, fee_rate, tip_height);

        assert!(
            result.is_err(),
            "expected Err for empty UTXO pool but got Ok"
        );

        // Accept either NoSpendableUtxos or InsufficientFunds — both are valid
        // responses when there is literally nothing to spend.
        match result.unwrap_err() {
            WalletError::InsufficientFunds { .. } | WalletError::NoSpendableUtxos => {}
            other => panic!(
                "expected InsufficientFunds or NoSpendableUtxos, got: {other:?}"
            ),
        }
    }
}
