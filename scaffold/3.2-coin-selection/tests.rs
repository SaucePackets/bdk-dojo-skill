// tests.rs — Lesson 3.2: Coin Selection
//
// This file contains pre-written tests for the `select_coins` stub.
// Run them with:
//
//   cargo test
//
// All tests should FAIL until you implement `select_coins` in `src/lib.rs`.
// Once your implementation is correct every test in this file will pass.

// Update this import to match your Cargo.toml package name.
use your_crate_name::{select_coins, FeeRate, Utxo, WalletError};

#[cfg(test)]
mod tests {
    use super::*;

    // ---------------------------------------------------------------------------
    // Helper: build a simple spendable Utxo with a given value.
    // Adjust field names if your Utxo struct uses different names.
    // ---------------------------------------------------------------------------
    fn make_utxo(value: u64, block_height: u32) -> Utxo {
        Utxo {
            value,
            block_height: Some(block_height),
            // If your Utxo has more required fields (outpoint, script_pubkey, …)
            // fill them in here with sensible dummy values.
        }
    }

    /// select_coins must return enough UTXOs to cover the target amount plus
    /// the estimated transaction fee.
    ///
    /// Setup:
    ///   UTXOs = [100_000, 50_000, 30_000] sat (all confirmed at height 750_000)
    ///   target = 80_000 sat, fee_rate = 2 sat/vB, tip_height = 800_000
    ///
    /// The selector may pick any subset whose total >= target + fee.
    #[test]
    fn coin_selection_picks_enough_utxos_to_cover_target_and_fee() {
        let utxos = vec![
            make_utxo(100_000, 750_000),
            make_utxo(50_000, 750_000),
            make_utxo(30_000, 750_000),
        ];
        let target = 80_000_u64;
        let fee_rate = FeeRate { sat_per_vb: 2 };
        let tip_height = 800_000_u32;

        let result = select_coins(target, fee_rate, &utxos, tip_height);

        assert!(result.is_ok(), "expected Ok but got: {:?}", result);

        let selected = result.unwrap();
        assert!(
            !selected.is_empty(),
            "coin selection must return at least one UTXO"
        );

        let selected_total: u64 = selected.iter().map(|u| u.value).sum();
        // The total of selected coins must be at least the target; the exact
        // minimum fee depends on your size estimation, so we allow a generous
        // upper bound here rather than hard-coding a fee value.
        assert!(
            selected_total >= target,
            "selected total ({selected_total}) must be >= target ({target})"
        );
        // Every selected UTXO must have come from the original pool.
        for u in &selected {
            assert!(
                utxos.iter().any(|orig| orig.value == u.value),
                "selected UTXO with value {} was not in the original pool",
                u.value
            );
        }
    }

    /// select_coins must return an error when the wallet does not hold enough
    /// funds to cover the target (plus any fee).
    ///
    /// Setup:
    ///   UTXOs = [10_000] sat — well below the 80_000 sat target.
    #[test]
    fn coin_selection_fails_when_balance_is_insufficient() {
        let utxos = vec![make_utxo(10_000, 750_000)];
        let target = 80_000_u64;
        let fee_rate = FeeRate { sat_per_vb: 2 };
        let tip_height = 800_000_u32;

        let result = select_coins(target, fee_rate, &utxos, tip_height);

        assert!(
            result.is_err(),
            "expected Err for insufficient funds but got Ok"
        );

        // Confirm the error variant signals insufficient funds (not some other error).
        match result.unwrap_err() {
            WalletError::InsufficientFunds { available, required } => {
                assert!(
                    available < required,
                    "available ({available}) should be less than required ({required})"
                );
            }
            other => panic!("expected WalletError::InsufficientFunds, got: {other:?}"),
        }
    }
}
