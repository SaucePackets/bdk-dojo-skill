// tests.rs — Lesson 4.3: Full Toy Send Flow (integration test)
//
// This file contains a single end-to-end integration test that wires together
// every concept from lessons 3.1 → 4.2:
//
//   WalletState  →  SyncEvent  →  balance()  →  propose_transaction()  →  review_plan()
//
// Run it with:
//
//   cargo test
//
// The test should FAIL until you have connected all the stubs into a working
// pipeline.  Once each piece is implemented correctly the full flow will pass.

// Update this import to match your Cargo.toml package name.
use your_crate_name::{
    propose_transaction, review_plan, FeeRate, SyncEvent, Utxo, WalletState,
};

#[cfg(test)]
mod tests {
    use super::*;

    // ---------------------------------------------------------------------------
    // Helper: construct a Utxo as the sync layer would produce it.
    // Adjust field names to match your actual Utxo struct definition.
    // ---------------------------------------------------------------------------
    fn make_utxo(value: u64, block_height: u32) -> Utxo {
        Utxo {
            value,
            block_height: Some(block_height),
            // Add any additional required fields with sensible dummy values.
        }
    }

    /// End-to-end flow:
    ///
    ///  1. Create a fresh WalletState with a starting scan height of 800_000.
    ///  2. Feed two SyncEvent::Found events — one UTXO of 500_000 sat and one
    ///     of 300_000 sat, both confirmed at block 799_990.
    ///  3. Feed two SyncEvent::Confirmed events to mark them spendable.
    ///  4. Assert wallet.balance() shows a positive spendable total.
    ///  5. Call propose_transaction with target=200_000 sat at 2 sat/vB.
    ///  6. Confirm the returned TxPlan is self-consistent.
    ///  7. Pass the plan through review_plan and assert outputs_match is true.
    #[test]
    fn full_toy_send_flow_sync_balance_select_propose_review() {
        // ── Step 1: create wallet ────────────────────────────────────────────
        let mut wallet = WalletState::new(800_000);

        // ── Step 2: sync — discover two UTXOs ────────────────────────────────
        let utxo_a = make_utxo(500_000, 799_990);
        let utxo_b = make_utxo(300_000, 799_990);

        wallet.apply_event(SyncEvent::Found(utxo_a.clone()));
        wallet.apply_event(SyncEvent::Found(utxo_b.clone()));

        // ── Step 3: confirm both UTXOs so they become spendable ───────────────
        wallet.apply_event(SyncEvent::Confirmed(utxo_a.clone()));
        wallet.apply_event(SyncEvent::Confirmed(utxo_b.clone()));

        // ── Step 4: verify balance ────────────────────────────────────────────
        let balance = wallet.balance();
        assert!(
            balance.total_spendable > 0,
            "total_spendable must be positive after confirming UTXOs; got {}",
            balance.total_spendable
        );
        // We added 500_000 + 300_000 = 800_000 sat of confirmed funds.
        assert_eq!(
            balance.total_spendable,
            800_000,
            "total_spendable should equal the sum of all confirmed UTXOs"
        );

        // ── Step 5: build a transaction proposal ─────────────────────────────
        let spendable = wallet.spendable_utxos();
        let target = 200_000_u64;
        let fee_rate = FeeRate { sat_per_vb: 2 };
        let tip_height = 800_000_u32;

        let result = propose_transaction(&spendable, target, fee_rate, tip_height);
        assert!(
            result.is_ok(),
            "propose_transaction must succeed with 800_000 sat available; got: {:?}",
            result
        );

        let plan = result.unwrap();

        // ── Step 6: verify the TxPlan is self-consistent ─────────────────────
        assert_eq!(
            plan.recipient_amount,
            target,
            "recipient_amount must equal requested target"
        );
        assert!(plan.fee > 0, "fee must be positive");
        assert!(!plan.selected.is_empty(), "selected UTXOs must be non-empty");

        let selected_total: u64 = plan.selected.iter().map(|u| u.value).sum();
        assert!(
            selected_total >= target + plan.fee,
            "selected total ({selected_total}) must cover target ({target}) + fee ({})",
            plan.fee
        );

        // ── Step 7: PSBT review ───────────────────────────────────────────────
        let policy = "wpkh([fingerprint/84'/0'/0']xpub…/0/*)";
        let review = review_plan(&plan, policy);

        assert!(
            review.outputs_match,
            "outputs_match must be true for a self-consistent plan"
        );
        assert!(
            review.fee_reasonable,
            "fee_reasonable must be true when fee > 0"
        );
        assert!(
            !review.unknown_recipients,
            "unknown_recipients must be false — all outputs are accounted for"
        );
    }
}
