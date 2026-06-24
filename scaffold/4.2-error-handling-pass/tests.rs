// tests.rs — Lesson 4.2: Error Handling Pass
//
// This file contains pre-written tests for the `WalletError` enum stub.
// Run them with:
//
//   cargo test
//
// All tests should FAIL until you implement/extend `WalletError` in `src/errors.rs`.
// Once your implementation is correct every test in this file will pass.

// Update this import to match your Cargo.toml package name.
use your_crate_name::WalletError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insufficient_funds_error_carries_amounts() {
        let err = WalletError::InsufficientFunds {
            needed: 50_000,
            available: 10_000,
        };

        match &err {
            WalletError::InsufficientFunds { needed, available } => {
                assert_eq!(*needed, 50_000);
                assert_eq!(*available, 10_000);
                assert!(available < needed);
            }
            other => panic!("expected WalletError::InsufficientFunds, got: {other:?}"),
        }

        let debug_str = format!("{err:?}");
        assert!(debug_str.contains("50000") || debug_str.contains("50_000"));
        assert!(debug_str.contains("10000") || debug_str.contains("10_000"));
    }

    #[test]
    fn dust_change_error_carries_amounts() {
        let err = WalletError::DustChange {
            change: 200,
            dust_limit: 546,
        };

        match &err {
            WalletError::DustChange { change, dust_limit } => {
                assert_eq!(*change, 200);
                assert_eq!(*dust_limit, 546);
                assert!(change < dust_limit);
            }
            other => panic!("expected WalletError::DustChange, got: {other:?}"),
        }
    }

    #[test]
    fn all_wallet_error_variants_are_constructable() {
        let _unsafe = WalletError::UnsafePsbt("fee too high".to_string());
        let _invalid = WalletError::InvalidDescriptor;
        let _unknown = WalletError::UnknownRecipient("bcrt1unknown".to_string());

        let _ = format!("{_unsafe:?}");
        let _ = format!("{_invalid:?}");
        let _ = format!("{_unknown:?}");
    }
}
