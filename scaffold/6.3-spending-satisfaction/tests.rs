// =============================================================================
// tests.rs — Lesson 6.3: Spending Satisfaction
// =============================================================================
//
// This file contains pre-written tests for the spending satisfaction kata.
// Your job is to implement the stubs in lib.rs so all tests pass.
//
// HOW TO USE:
//   1. Update the `use` import below to match your Cargo.toml [package] name.
//   2. Implement the following in your crate (in addition to the Policy enum
//      from lessons 6.1/6.2):
//
//        #[derive(Debug, PartialEq)]
//        pub struct SatisfactionSummary {
//            pub signatures_required: usize,
//            pub timelock_required:   Option<u32>,
//            pub preimage_required:   bool,
//            pub or_branches_count:   usize,
//        }
//
//        pub fn analyze_satisfaction(policy: &Policy) -> SatisfactionSummary
//        pub fn can_satisfy_with(
//            policy: &Policy,
//            available_signatures: usize,
//            current_height: u32,
//            has_preimage: bool,
//        ) -> bool
//        pub fn describe_satisfaction(policy: &Policy) -> String
//
//   3. Run: cargo test
//
// SEMANTICS:
//   analyze_satisfaction:
//     - Key(_)            → { sigs: 1, timelock: None, preimage: false, or_branches: 0 }
//     - And([Key, Older]) → { sigs: 1, timelock: Some(n), preimage: false, or_branches: 0 }
//     - Thresh(k, keys)   → { sigs: k, timelock: None, preimage: false, or_branches: 0 }
//     - Or(children)      → { sigs: min_sigs_across_children, or_branches: children.len() }
//     - HashLock(_)       → { sigs: 0, timelock: None, preimage: true, or_branches: 0 }
//
//   can_satisfy_with returns true when ALL of the following hold:
//     - available_signatures >= signatures_required
//     - if timelock_required == Some(n): current_height >= n
//     - if preimage_required: has_preimage == true
//
//   describe_satisfaction returns a non-empty string for any Policy.
// =============================================================================

// Update this import to match your Cargo.toml package name.
use your_crate_name::{analyze_satisfaction, can_satisfy_with, describe_satisfaction, Policy, SatisfactionSummary};

#[cfg(test)]
mod tests {
    use super::*;

    // ── analyze_satisfaction ─────────────────────────────────────────────────

    /// A bare Key requires exactly one signature, no timelock, no preimage,
    /// and has no or-branches.
    #[test]
    fn single_key_requires_one_signature_no_timelock() {
        let policy = Policy::Key("alice".into());
        let summary = analyze_satisfaction(&policy);

        assert_eq!(
            summary.signatures_required, 1,
            "Policy::Key should require exactly 1 signature"
        );
        assert_eq!(
            summary.timelock_required, None,
            "Policy::Key should have no timelock requirement"
        );
        assert!(
            !summary.preimage_required,
            "Policy::Key should not require a hash preimage"
        );
        assert_eq!(
            summary.or_branches_count, 0,
            "Policy::Key has no or-branches"
        );
    }

    /// And(Key, Older(n)) requires 1 sig AND the chain tip must be >= n.
    #[test]
    fn timelocked_policy_requires_height_and_signature() {
        let policy = Policy::And(vec![Policy::Key("bob".into()), Policy::Older(1000)]);
        let summary = analyze_satisfaction(&policy);

        assert_eq!(
            summary.signatures_required, 1,
            "And(Key, Older) should still require 1 signature"
        );
        assert_eq!(
            summary.timelock_required,
            Some(1000),
            "And(Key, Older(1000)) should record timelock_required = Some(1000)"
        );
        assert!(
            !summary.preimage_required,
            "And(Key, Older) does not require a hash preimage"
        );
        assert_eq!(
            summary.or_branches_count, 0,
            "And(Key, Older) has no or-branches"
        );
    }

    /// Thresh(2, [Key, Key, Key]) requires exactly 2 signatures.
    #[test]
    fn multisig_2_of_3_requires_two_signatures() {
        let policy = Policy::Thresh(
            2,
            vec![
                Policy::Key("a".into()),
                Policy::Key("b".into()),
                Policy::Key("c".into()),
            ],
        );
        let summary = analyze_satisfaction(&policy);

        assert_eq!(
            summary.signatures_required, 2,
            "Thresh(2, [Key, Key, Key]) should require 2 signatures"
        );
        assert_eq!(
            summary.timelock_required, None,
            "A plain multisig has no timelock"
        );
        assert!(
            !summary.preimage_required,
            "A plain multisig does not require a hash preimage"
        );
    }

    /// Or(Key(a), Key(b)) should expose 2 branches.
    /// Spending via either branch alone should satisfy the policy.
    #[test]
    fn or_policy_can_be_satisfied_via_either_branch() {
        let policy = Policy::Or(vec![Policy::Key("alice".into()), Policy::Key("bob".into())]);
        let summary = analyze_satisfaction(&policy);

        assert_eq!(
            summary.or_branches_count, 2,
            "Or(Key, Key) should report 2 or-branches"
        );
        // The easiest branch requires only 1 signature.
        assert_eq!(
            summary.signatures_required, 1,
            "Or(Key, Key) — best branch requires 1 signature"
        );
        assert_eq!(
            summary.timelock_required, None,
            "Or(Key, Key) — neither branch has a timelock"
        );
    }

    // ── can_satisfy_with ─────────────────────────────────────────────────────

    /// Comprehensive test covering all branching conditions.
    #[test]
    fn can_satisfy_with_checks_all_conditions() {
        // ── bare key ─────────────────────────────────────────────────────────

        let key = Policy::Key("alice".into());

        // 1 sig, any height → satisfied
        assert!(
            can_satisfy_with(&key, 1, 0, false),
            "Key with 1 available sig should be satisfiable"
        );

        // 0 sigs → not satisfied
        assert!(
            !can_satisfy_with(&key, 0, 0, false),
            "Key with 0 available sigs should NOT be satisfiable"
        );

        // ── timelocked And ───────────────────────────────────────────────────

        let timelocked = Policy::And(vec![Policy::Key("bob".into()), Policy::Older(1000)]);

        // Correct sig count but height not yet reached → not satisfied
        assert!(
            !can_satisfy_with(&timelocked, 1, 500, false),
            "And(Key, Older(1000)) with height 500 should NOT be satisfiable (timelock unmet)"
        );

        // Height exactly meets the timelock requirement → satisfied
        assert!(
            can_satisfy_with(&timelocked, 1, 1000, false),
            "And(Key, Older(1000)) with height 1000 should be satisfiable"
        );

        // Height exceeds requirement → also satisfied
        assert!(
            can_satisfy_with(&timelocked, 1, 2000, false),
            "And(Key, Older(1000)) with height 2000 should be satisfiable"
        );

        // Timelock met but no signature → not satisfied
        assert!(
            !can_satisfy_with(&timelocked, 0, 1000, false),
            "And(Key, Older(1000)) with height 1000 but 0 sigs should NOT be satisfiable"
        );

        // ── multisig ─────────────────────────────────────────────────────────

        let multisig = Policy::Thresh(
            2,
            vec![
                Policy::Key("a".into()),
                Policy::Key("b".into()),
                Policy::Key("c".into()),
            ],
        );

        // 2 sigs → satisfied
        assert!(
            can_satisfy_with(&multisig, 2, 0, false),
            "Thresh(2, ...) with 2 sigs should be satisfiable"
        );

        // Only 1 sig → not satisfied
        assert!(
            !can_satisfy_with(&multisig, 1, 0, false),
            "Thresh(2, ...) with only 1 sig should NOT be satisfiable"
        );

        // ── hash lock ────────────────────────────────────────────────────────

        let hash_lock = Policy::HashLock([0u8; 32]);

        // Has preimage → satisfied (no sig needed)
        assert!(
            can_satisfy_with(&hash_lock, 0, 0, true),
            "HashLock with preimage should be satisfiable"
        );

        // No preimage → not satisfied
        assert!(
            !can_satisfy_with(&hash_lock, 0, 0, false),
            "HashLock without preimage should NOT be satisfiable"
        );

        // ── describe_satisfaction ─────────────────────────────────────────────

        let desc = describe_satisfaction(&Policy::Key("carol".into()));
        assert!(
            !desc.is_empty(),
            "describe_satisfaction should return a non-empty string"
        );

        let desc2 = describe_satisfaction(&timelocked);
        assert!(
            !desc2.is_empty(),
            "describe_satisfaction for a timelocked policy should return a non-empty string"
        );
    }
}
