// =============================================================================
// tests.rs — Lesson 6.2: Policy Composition
// =============================================================================
//
// This file contains pre-written tests for the policy composition kata.
// Your job is to implement the stubs in lib.rs so all tests pass.
//
// HOW TO USE:
//   1. Update the `use` import below to match your Cargo.toml [package] name.
//   2. Implement the following in your crate (in addition to the Policy enum
//      and helpers from lesson 6.1):
//
//        pub fn compose_policy(pattern: &str, params: &[&str]) -> Policy
//        pub fn describe_policy(policy: &Policy) -> String
//        pub fn extract_timelocks(policy: &Policy) -> Vec<u32>
//        pub fn required_signatures(policy: &Policy) -> usize
//
//   3. Run: cargo test
//
// SEMANTICS:
//   compose_policy("multisig_with_recovery", &["alice", "bob", "carol", "1000"])
//     should produce a policy tree shaped like:
//       Or(
//         Thresh(2, [Key("alice"), Key("bob"), Key("carol")]),
//         And([Key("carol"), Older(1000)]),
//       )
//     i.e. a 2-of-3 with a recovery path gated behind a timelock.
//
//   describe_policy returns a non-empty human-readable string for any input.
//
//   extract_timelocks collects all Older/After values found anywhere in the
//   tree and returns them deduplicated (order need not be sorted).
//
//   required_signatures:
//     - Thresh(k, _) → k
//     - Key(_)       → 1
//     - And(children) → sum of required_signatures for each child
//     - Or(children)  → min required_signatures across children (best path)
//     - Older/After/HashLock → 0 (not signature requirements)
// =============================================================================

// Update this import to match your Cargo.toml package name.
use your_crate_name::{
    compose_policy, count_keys, describe_policy, extract_timelocks, is_timelocked,
    required_signatures, Policy,
};

#[cfg(test)]
mod tests {
    use super::*;

    /// compose_policy("multisig_with_recovery", ...) should build a tree that
    /// contains both a multi-key path and a timelock recovery path.
    #[test]
    fn compose_multisig_with_recovery_produces_correct_tree() {
        // params: primary signers + recovery signer + timelock height
        let policy = compose_policy(
            "multisig_with_recovery",
            &["alice", "bob", "carol", "1000"],
        );

        // The composed policy must be timelocked (recovery path uses Older/After).
        assert!(
            is_timelocked(&policy),
            "multisig_with_recovery policy must contain a timelock (recovery path)"
        );

        // The composed policy must reference more than one key.
        assert!(
            count_keys(&policy) > 1,
            "multisig_with_recovery policy must involve more than one key"
        );

        // Sanity: at least the three primary signers are present, so >= 3 keys
        // (the recovery key may be shared, but the total should be >= 3).
        assert!(
            count_keys(&policy) >= 3,
            "multisig_with_recovery with three signers should have at least 3 key references"
        );
    }

    /// describe_policy must return a non-empty, human-readable string for any
    /// Policy variant — callers should never receive an empty description.
    #[test]
    fn describe_policy_returns_human_readable_summary_for_known_patterns() {
        // Bare key
        let key_desc = describe_policy(&Policy::Key("alice".into()));
        assert!(
            !key_desc.is_empty(),
            "describe_policy should return a non-empty string for Policy::Key"
        );

        // Threshold / multisig
        let multisig = Policy::Thresh(
            2,
            vec![
                Policy::Key("a".into()),
                Policy::Key("b".into()),
                Policy::Key("c".into()),
            ],
        );
        let multi_desc = describe_policy(&multisig);
        assert!(
            !multi_desc.is_empty(),
            "describe_policy should return a non-empty string for Thresh"
        );

        // Timelocked And
        let timelocked = Policy::And(vec![Policy::Key("x".into()), Policy::Older(1000)]);
        let time_desc = describe_policy(&timelocked);
        assert!(
            !time_desc.is_empty(),
            "describe_policy should return a non-empty string for And(Key, Older)"
        );
    }

    /// extract_timelocks should find every Older/After value in the tree and
    /// return each distinct value exactly once.
    #[test]
    fn extract_timelocks_finds_all_distinct_values() {
        // Two different timelock types in a flat And
        let policy = Policy::And(vec![Policy::Older(1000), Policy::After(500)]);
        let mut locks = extract_timelocks(&policy);
        locks.sort(); // normalise order for assertion

        assert_eq!(
            locks.len(),
            2,
            "Should find exactly 2 distinct timelock values in And(Older(1000), After(500))"
        );
        assert!(locks.contains(&1000), "Should contain Older value 1000");
        assert!(locks.contains(&500), "Should contain After value 500");

        // Duplicate Older values should be deduplicated
        let dupes = Policy::And(vec![Policy::Older(1000), Policy::Older(1000)]);
        let deduped = extract_timelocks(&dupes);
        assert_eq!(
            deduped.len(),
            1,
            "Duplicate timelock values should be deduplicated; expected 1, got {}",
            deduped.len()
        );
        assert!(deduped.contains(&1000), "Deduplicated result should contain 1000");

        // No timelocks in a key-only policy
        let no_locks = Policy::Key("alice".into());
        assert!(
            extract_timelocks(&no_locks).is_empty(),
            "A bare Key policy should yield no timelocks"
        );

        // Nested timelock buried inside an Or
        let nested = Policy::Or(vec![
            Policy::Key("a".into()),
            Policy::And(vec![Policy::Key("b".into()), Policy::Older(750)]),
        ]);
        let nested_locks = extract_timelocks(&nested);
        assert_eq!(nested_locks.len(), 1, "Should find 1 timelock in the nested Or");
        assert!(nested_locks.contains(&750), "Should find Older(750)");
    }

    /// required_signatures for a Thresh(k, ...) should return k.
    /// For a bare Key it should return 1.
    #[test]
    fn multisig_2_of_3_requires_2_signatures() {
        // Thresh(2, [Key, Key, Key]) → 2
        let multisig = Policy::Thresh(
            2,
            vec![
                Policy::Key("a".into()),
                Policy::Key("b".into()),
                Policy::Key("c".into()),
            ],
        );
        assert_eq!(
            required_signatures(&multisig),
            2,
            "A 2-of-3 Thresh should require 2 signatures"
        );

        // Bare key → 1
        let single = Policy::Key("alice".into());
        assert_eq!(
            required_signatures(&single),
            1,
            "A bare Key should require 1 signature"
        );

        // Thresh(1, [Key, Key]) — 1-of-2 → 1
        let one_of_two = Policy::Thresh(1, vec![Policy::Key("a".into()), Policy::Key("b".into())]);
        assert_eq!(
            required_signatures(&one_of_two),
            1,
            "Thresh(1, [Key, Key]) should require 1 signature"
        );

        // Thresh(3, [Key, Key, Key]) — 3-of-3 → 3
        let three_of_three = Policy::Thresh(
            3,
            vec![
                Policy::Key("a".into()),
                Policy::Key("b".into()),
                Policy::Key("c".into()),
            ],
        );
        assert_eq!(
            required_signatures(&three_of_three),
            3,
            "Thresh(3, [Key, Key, Key]) should require 3 signatures"
        );
    }
}
