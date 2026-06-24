// lesson_3_3_dust_and_change_policy.rs — Lesson 3.3: Dust and Change Policy
//
// This file contains pre-written tests for the `decide_change` stub.
// Run them with:
//
//   cargo test
//
// All tests should FAIL until you implement `decide_change` in `src/change.rs`.
// Once your implementation is correct every test in this file will pass.

// This course standardizes on package `bdk-dojo`, imported as `bdk_dojo`.
use bdk_dojo::{decide_change, ChangeDecision, DUST_LIMIT};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dust_change_is_added_to_fee_instead_of_output() {
        assert_eq!(decide_change(0), ChangeDecision::NoChange);
        assert_eq!(decide_change(218), ChangeDecision::AddToFee(218));
        assert_eq!(
            decide_change(DUST_LIMIT - 1),
            ChangeDecision::AddToFee(DUST_LIMIT - 1)
        );
    }

    #[test]
    fn change_at_or_above_dust_limit_is_kept() {
        assert_eq!(
            decide_change(DUST_LIMIT),
            ChangeDecision::Change(DUST_LIMIT)
        );
        assert_eq!(
            decide_change(DUST_LIMIT + 1),
            ChangeDecision::Change(DUST_LIMIT + 1)
        );
        assert_eq!(decide_change(49_718), ChangeDecision::Change(49_718));
    }
}
