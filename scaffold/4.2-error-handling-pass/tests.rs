// tests.rs — Lesson 4.2: Error Handling Pass
//
// This file contains pre-written tests for the `WalletError` enum stub.
// Run them with:
//
//   cargo test
//
// All tests should FAIL until you implement the WalletError enum (and its
// Debug / Display impls) in `src/lib.rs`.
// Once your implementation is correct every test in this file will pass.

// Update this import to match your Cargo.toml package name.
use your_crate_name::WalletError;

#[cfg(test)]
mod tests {
    use super::*;

    /// WalletError::InsufficientFunds must carry both the available balance
    /// and the required amount so callers can surface a helpful message.
    #[test]
    fn insufficient_funds_error_carries_amounts() {
        let err = WalletError::InsufficientFunds {
            available: 10_000,
            required: 50_000,
        };

        // Destructure the variant to verify both fields are accessible.
        match &err {
            WalletError::InsufficientFunds { available, required } => {
                assert_eq!(*available, 10_000, "available field should be 10_000");
                assert_eq!(*required, 50_000, "required field should be 50_000");
                assert!(
                    available < required,
                    "available ({available}) must be less than required ({required})"
                );
            }
            other => panic!(
                "expected WalletError::InsufficientFunds, got: {other:?}"
            ),
        }

        // The error must implement Debug so it can be printed in test output.
        let debug_str = format!("{err:?}");
        assert!(
            !debug_str.is_empty(),
            "Debug output must not be empty"
        );
        // Both numeric values should appear somewhere in the debug string.
        assert!(
            debug_str.contains("10000") || debug_str.contains("10_000"),
            "Debug output should mention the available balance; got: {debug_str}"
        );
        assert!(
            debug_str.contains("50000") || debug_str.contains("50_000"),
            "Debug output should mention the required amount; got: {debug_str}"
        );
    }

    /// WalletError::DustOutput must carry the offending output amount so the
    /// caller knows how small the output was.
    #[test]
    fn dust_output_error_carries_amount() {
        let err = WalletError::DustOutput { amount: 200 };

        // Destructure the variant to verify the field is accessible.
        match &err {
            WalletError::DustOutput { amount } => {
                assert_eq!(*amount, 200, "amount field should be 200");
                assert!(
                    *amount < 546,
                    "a dust output of {amount} sat should be below the 546 sat dust limit"
                );
            }
            other => panic!("expected WalletError::DustOutput, got: {other:?}"),
        }

        // The error must implement Debug.
        let debug_str = format!("{err:?}");
        assert!(!debug_str.is_empty(), "Debug output must not be empty");
        assert!(
            debug_str.contains("200"),
            "Debug output should mention the dust amount; got: {debug_str}"
        );
    }

    /// Smoke-test the remaining variants to ensure the enum is complete.
    #[test]
    fn all_wallet_error_variants_are_constructable() {
        // NoSpendableUtxos — unit variant
        let _e1 = WalletError::NoSpendableUtxos;

        // UnknownPolicy — tuple variant carrying a String
        let _e2 = WalletError::UnknownPolicy("wpkh(unknown)".to_string());

        // Just check they can be Debug-formatted without panicking.
        let _ = format!("{_e1:?}");
        let _ = format!("{_e2:?}");
    }
}
