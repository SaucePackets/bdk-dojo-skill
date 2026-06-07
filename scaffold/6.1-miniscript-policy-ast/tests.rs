// =============================================================================
// tests.rs — Lesson 6.1: Miniscript Policy AST
// =============================================================================
//
// This file contains pre-written tests for the Miniscript Policy AST kata.
// Your job is to implement the stubs in lib.rs so all tests pass.
//
// HOW TO USE:
//   1. Update the `use` import below to match your Cargo.toml [package] name.
//   2. Implement the following in your crate:
//
//        #[derive(Debug, PartialEq)]
//        pub enum Policy {
//            Key(String),
//            Older(u32),
//            After(u32),
//            And(Vec<Policy>),
//            Or(Vec<Policy>),
//            Thresh(u32, Vec<Policy>),
//            HashLock([u8; 32]),
//        }
//
//        pub fn policy_kind(policy: &Policy) -> &'static str
//        pub fn count_keys(policy: &Policy) -> usize
//        pub fn is_timelocked(policy: &Policy) -> bool
//        pub fn is_multisig(policy: &Policy) -> bool
//
//   3. Run: cargo test
//
// SEMANTICS:
//   - policy_kind returns a human-readable label:
//       Key        → "single key"
//       Thresh     → "multisig" (if all children are keys) or "threshold"
//       And        → "and"
//       Or         → "or"
//       Older      → "relative timelock"
//       After      → "absolute timelock"
//       HashLock   → "hash lock"
//   - count_keys recursively counts all Key leaves.
//   - is_timelocked returns true if the policy tree contains any Older or After
//     node at any depth.
//   - is_multisig returns true if the top-level node is a Thresh whose children
//     are all Key leaves (classic k-of-n).
// =============================================================================

// Update this import to match your Cargo.toml package name.
use your_crate_name::{count_keys, is_multisig, is_timelocked, policy_kind, Policy};

#[cfg(test)]
mod tests {
    use super::*;

    // ── policy_kind ──────────────────────────────────────────────────────────

    /// A bare Key node should be labelled "single key".
    #[test]
    fn policy_kind_classifies_single_key_as_single_key() {
        let p = Policy::Key("alice".into());
        assert_eq!(
            policy_kind(&p),
            "single key",
            "Policy::Key should return \"single key\""
        );
    }

    /// A Thresh node whose children are all Keys should be labelled "multisig".
    #[test]
    fn policy_kind_identifies_threshold_as_multisig() {
        let p = Policy::Thresh(
            2,
            vec![
                Policy::Key("a".into()),
                Policy::Key("b".into()),
                Policy::Key("c".into()),
            ],
        );
        assert_eq!(
            policy_kind(&p),
            "multisig",
            "Thresh(2, [Key, Key, Key]) should return \"multisig\""
        );
    }

    // ── count_keys ───────────────────────────────────────────────────────────

    /// count_keys should sum all Key leaves in the tree.
    #[test]
    fn count_keys_counts_all_keys_in_nested_policy() {
        // Flat And: 2 keys
        let flat = Policy::And(vec![Policy::Key("a".into()), Policy::Key("b".into())]);
        assert_eq!(
            count_keys(&flat),
            2,
            "And(Key, Key) should have 2 keys"
        );

        // Nested: And(Key(a), Or(Key(b), Key(c))) → 3 keys total
        let nested = Policy::And(vec![
            Policy::Key("a".into()),
            Policy::Or(vec![Policy::Key("b".into()), Policy::Key("c".into())]),
        ]);
        assert_eq!(
            count_keys(&nested),
            3,
            "And(Key(a), Or(Key(b), Key(c))) should have 3 keys"
        );

        // Single key leaf
        let single = Policy::Key("solo".into());
        assert_eq!(count_keys(&single), 1, "A bare Key node counts as 1");

        // No keys in a timelock node
        let timelock_only = Policy::Older(1000);
        assert_eq!(
            count_keys(&timelock_only),
            0,
            "An Older node contains 0 keys"
        );
    }

    // ── is_timelocked ────────────────────────────────────────────────────────

    /// is_timelocked should return true when any node in the tree is a timelock.
    #[test]
    fn timelock_detection_works_on_nested_policy() {
        // And(Key, Older) — has a timelock
        let locked = Policy::And(vec![
            Policy::Key("a".into()),
            Policy::Older(1000),
        ]);
        assert!(
            is_timelocked(&locked),
            "And(Key, Older(1000)) should be considered timelocked"
        );

        // And(Key, After) — absolute timelock also counts
        let abs_locked = Policy::And(vec![
            Policy::Key("a".into()),
            Policy::After(700_000),
        ]);
        assert!(
            is_timelocked(&abs_locked),
            "And(Key, After(700_000)) should be considered timelocked"
        );

        // Bare key — not timelocked
        let no_lock = Policy::Key("a".into());
        assert!(
            !is_timelocked(&no_lock),
            "Policy::Key should not be considered timelocked"
        );

        // Nested: Or(Key, And(Key, Older)) — timelock is buried one level deeper
        let deeply_locked = Policy::Or(vec![
            Policy::Key("x".into()),
            Policy::And(vec![Policy::Key("y".into()), Policy::Older(500)]),
        ]);
        assert!(
            is_timelocked(&deeply_locked),
            "Or(Key, And(Key, Older)) should be considered timelocked"
        );
    }

    // ── is_multisig ──────────────────────────────────────────────────────────

    /// is_multisig should return true only for top-level Thresh-of-Keys.
    #[test]
    fn is_multisig_treats_multi_key_threshold_as_multisig() {
        // Classic 2-of-3
        let multisig = Policy::Thresh(
            2,
            vec![
                Policy::Key("a".into()),
                Policy::Key("b".into()),
                Policy::Key("c".into()),
            ],
        );
        assert!(
            is_multisig(&multisig),
            "Thresh(2, [Key, Key, Key]) should be is_multisig == true"
        );

        // And(Key, Older) is NOT a multisig
        let not_multisig = Policy::And(vec![
            Policy::Key("a".into()),
            Policy::Older(100),
        ]);
        assert!(
            !is_multisig(&not_multisig),
            "And(Key, Older) should not be classified as multisig"
        );

        // A bare Key is not a multisig
        let bare_key = Policy::Key("a".into());
        assert!(
            !is_multisig(&bare_key),
            "A single Key should not be classified as multisig"
        );
    }
}
