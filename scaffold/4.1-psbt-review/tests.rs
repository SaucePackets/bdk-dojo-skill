// tests.rs — Lesson 4.1: PSBT Review
//
// This file contains pre-written tests for the `review_plan` stub.
// Run them with:
//
//   cargo test
//
// All tests should FAIL until you implement `review_plan` in `src/lib.rs`.
// Once your implementation is correct every test in this file will pass.

// Update this import to match your Cargo.toml package name.
use your_crate_name::{review_plan, FeeRate, PsbtReview, TxPlan, Utxo};

#[cfg(test)]
mod tests {
    use super::*;

    // ---------------------------------------------------------------------------
    // Helpers: build minimal TxPlan values for testing.
    // Adjust Utxo / TxPlan field names to match your struct definitions.
    // ---------------------------------------------------------------------------

    fn make_utxo(value: u64) -> Utxo {
        Utxo {
            value,
            block_height: Some(750_000),
            // Add any additional required fields with sensible dummy values.
        }
    }

    /// Build a "healthy" TxPlan: positive fee, explicit change, no surprises.
    fn valid_plan() -> TxPlan {
        TxPlan {
            selected: vec![make_utxo(100_000)],
            recipient_amount: 50_000,
            fee: 282,           // 141 vB × 2 sat/vB
            change: Some(49_718), // 100_000 - 50_000 - 282
        }
    }

    /// Build a degenerate TxPlan with zero fee (miner would never confirm this).
    fn zero_fee_plan() -> TxPlan {
        TxPlan {
            selected: vec![make_utxo(100_000)],
            recipient_amount: 50_000,
            fee: 0,
            change: Some(50_000),
        }
    }

    // -------------------------------------------------------------------------

    /// A well-formed plan must pass every review gate.
    ///
    /// Expected:
    ///   outputs_match:      true  — amounts are self-consistent
    ///   fee_reasonable:     true  — fee is positive
    ///   change_is_ours:     true  — change output belongs to our wallet
    ///   unknown_recipients: false — no surprises in the output set
    #[test]
    fn valid_plan_passes_psbt_review() {
        let plan = valid_plan();
        // The wallet_policy string identifies our descriptor / output policy.
        // Pass a simple sentinel; your implementation may use it to verify
        // the change output derivation path.
        let review: PsbtReview = review_plan(&plan, "wpkh([fingerprint/84'/0'/0']xpub…/0/*)");

        assert!(
            review.outputs_match,
            "outputs_match should be true for a self-consistent plan"
        );
        assert!(
            review.fee_reasonable,
            "fee_reasonable should be true when fee > 0"
        );
        assert!(
            review.change_is_ours,
            "change_is_ours should be true when change is present and belongs to wallet"
        );
        assert!(
            !review.unknown_recipients,
            "unknown_recipients should be false for a known-good plan"
        );
    }

    /// A plan whose fee is zero should fail the fee_reasonable check while
    /// still being self-consistent on amounts.
    #[test]
    fn plan_with_zero_fee_fails_review() {
        let plan = zero_fee_plan();
        let review: PsbtReview = review_plan(&plan, "wpkh([fingerprint/84'/0'/0']xpub…/0/*)");

        assert!(
            !review.fee_reasonable,
            "fee_reasonable should be false when fee == 0"
        );

        // The amounts still add up correctly even with zero fee.
        assert!(
            review.outputs_match,
            "outputs_match should still be true even with a zero fee"
        );
    }
}
