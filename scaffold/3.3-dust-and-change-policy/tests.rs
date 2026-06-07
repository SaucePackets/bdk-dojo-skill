// tests.rs — Lesson 3.3: Dust and Change Policy
//
// This file contains pre-written tests for the `change_decision` stub.
// Run them with:
//
//   cargo test
//
// All tests should FAIL until you implement `change_decision` in `src/lib.rs`.
// Once your implementation is correct every test in this file will pass.

// Update this import to match your Cargo.toml package name.
use your_crate_name::{change_decision, ChangeDecision, DUST_LIMIT};

#[cfg(test)]
mod tests {
    use super::*;

    /// When the change amount (selected_total - target - fee) is strictly
    /// greater than DUST_LIMIT, the function must return AddChange(amount).
    ///
    /// Also verifies the NoChange path for a zero remainder.
    #[test]
    fn change_above_dust_is_added() {
        // Remainder = 100_000 - 50_000 - 282 = 49_718 sat — well above 546.
        let result = change_decision(100_000, 50_000, 282);
        assert_eq!(
            result,
            ChangeDecision::AddChange(49_718),
            "remainder 49_718 is above DUST_LIMIT ({DUST_LIMIT}), expected AddChange(49_718)"
        );

        // Sanity-check: a remainder of exactly zero must NOT add change.
        let result_zero = change_decision(50_000, 50_000, 0);
        assert_eq!(
            result_zero,
            ChangeDecision::NoChange,
            "remainder 0 should produce NoChange, not AddChange(0)"
        );
    }

    /// When the change amount falls at or below DUST_LIMIT the fee would cost
    /// more than the output is worth, so the function must return NoChange and
    /// effectively donate the tiny remainder as an additional fee.
    #[test]
    fn change_below_dust_is_dropped() {
        // Remainder = 50_500 - 50_000 - 282 = 218 sat — below DUST_LIMIT of 546.
        let result = change_decision(50_500, 50_000, 282);
        assert_eq!(
            result,
            ChangeDecision::NoChange,
            "remainder 218 is below DUST_LIMIT ({DUST_LIMIT}), expected NoChange"
        );

        // Boundary: exactly DUST_LIMIT should still be dropped (dust limit is
        // the minimum *spendable* output; an output equal to it is still dust).
        let result_at_limit = change_decision(50_000 + DUST_LIMIT, 50_000, 0);
        assert_eq!(
            result_at_limit,
            ChangeDecision::NoChange,
            "remainder equal to DUST_LIMIT ({DUST_LIMIT}) should produce NoChange"
        );

        // One satoshi above the limit — just large enough to create change.
        let result_above = change_decision(50_000 + DUST_LIMIT + 1, 50_000, 0);
        assert_eq!(
            result_above,
            ChangeDecision::AddChange(DUST_LIMIT + 1),
            "remainder one sat above DUST_LIMIT should produce AddChange({})",
            DUST_LIMIT + 1
        );
    }
}
