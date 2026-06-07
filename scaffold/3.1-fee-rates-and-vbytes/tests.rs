// tests.rs — Lesson 3.1: Fee Rates and Vbytes
//
// This file contains pre-written tests for the fee calculation stub.
// Run them with:
//
//   cargo test
//
// All tests should FAIL until you implement the `fee` function in `src/lib.rs`.
// Once your implementation is correct every test in this file will pass.

// Update this import to match your Cargo.toml package name.
use your_crate_name::{fee, FeeRate, TxSizeEstimate};

#[cfg(test)]
mod tests {
    use super::*;

    /// fee() must multiply vbytes by sat_per_vb exactly.
    ///
    /// Concrete cases:
    ///   141 vB × 2 sat/vB  = 282 sat
    ///   200 vB × 10 sat/vB = 2 000 sat
    ///     0 vB × 5 sat/vB  = 0 sat   (zero-size edge case)
    #[test]
    fn fee_is_vbytes_times_fee_rate() {
        // Standard P2WPKH-input + 1 P2WPKH-output transaction: 141 vB
        assert_eq!(
            fee(
                TxSizeEstimate { vbytes: 141 },
                FeeRate { sat_per_vb: 2 }
            ),
            282,
            "141 vB at 2 sat/vB should cost 282 sat"
        );

        // Larger transaction body at an elevated fee rate
        assert_eq!(
            fee(
                TxSizeEstimate { vbytes: 200 },
                FeeRate { sat_per_vb: 10 }
            ),
            2_000,
            "200 vB at 10 sat/vB should cost 2 000 sat"
        );

        // Zero-size input must never produce a non-zero fee
        assert_eq!(
            fee(
                TxSizeEstimate { vbytes: 0 },
                FeeRate { sat_per_vb: 5 }
            ),
            0,
            "0 vB at any rate should cost 0 sat"
        );
    }
}
