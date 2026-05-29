/// src/miniscript_ast.rs
///
/// Toy policy AST — models Miniscript-like spending conditions.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Policy {
    /// A single key must sign
    Key(String),
    /// Relative timelock (CSV) — must wait N blocks
    Older(u32),
    /// Absolute timelock (CLTV) — must wait until block height N
    After(u32),
    /// All sub-policies must be satisfied
    And(Vec<Policy>),
    /// Any sub-policy can be satisfied
    Or(Vec<Policy>),
    /// At least k of n sub-policies must be satisfied
    Thresh(u32, Vec<Policy>),
    /// A hash preimage must be revealed
    HashLock([u8; 32]),
}

/// Classify a policy by its top-level shape.
pub fn policy_kind(policy: &Policy) -> &'static str {
    match policy {
        Policy::Key(_) => "single key",
        Policy::Older(_) | Policy::After(_) => "timelocked",
        Policy::And(_) => "composite",
        Policy::Or(_) => "composite",
        Policy::Thresh(_, _) => "multisig",
        Policy::HashLock(_) => "hash locked",
    }
}

/// Count all keys referenced anywhere in the policy tree.
pub fn count_keys(policy: &Policy) -> usize {
    match policy {
        Policy::Key(_) => 1,
        Policy::Older(_) | Policy::After(_) => 0,
        Policy::HashLock(_) => 0,
        Policy::And(children) => children.iter().map(count_keys).sum(),
        Policy::Or(children) => children.iter().map(count_keys).sum(),
        Policy::Thresh(_, children) => children.iter().map(count_keys).sum(),
    }
}

/// Returns true if any branch has an Older or After timelock.
pub fn is_timelocked(policy: &Policy) -> bool {
    match policy {
        Policy::Older(_) | Policy::After(_) => true,
        Policy::Key(_) | Policy::HashLock(_) => false,
        Policy::And(children) | Policy::Or(children) => children.iter().any(is_timelocked),
        Policy::Thresh(_, children) => children.iter().any(is_timelocked),
    }
}

/// Returns true if the policy is a Thresh with multiple keys and no other types.
pub fn is_multisig(policy: &Policy) -> bool {
    match policy {
        Policy::Thresh(k, children) => {
            *k >= 2 && children.len() >= 2 && children.iter().all(|c| matches!(c, Policy::Key(_)))
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(name: &str) -> Policy {
        Policy::Key(name.to_string())
    }

    #[test]
    fn policy_kind_classifies_single_key_as_single_key() {
        assert_eq!(policy_kind(&key("alice")), "single key");
    }

    #[test]
    fn policy_kind_identifies_threshold_as_multisig() {
        let pol = Policy::Thresh(2, vec![key("a"), key("b"), key("c")]);
        assert_eq!(policy_kind(&pol), "multisig");
    }

    #[test]
    fn count_keys_counts_all_keys_in_nested_policy() {
        let pol = Policy::And(vec![
            key("alice"),
            Policy::Or(vec![key("bob"), key("carol")]),
        ]);
        assert_eq!(count_keys(&pol), 3);
    }

    #[test]
    fn timelock_detection_works_on_nested_policy() {
        let pol = Policy::Thresh(
            2,
            vec![
                Policy::And(vec![key("alice"), Policy::Older(100)]),
                key("bob"),
            ],
        );
        assert!(is_timelocked(&pol));
    }

    #[test]
    fn is_multisig_treats_multi_key_threshold_as_multisig() {
        let pol = Policy::Thresh(2, vec![key("a"), key("b"), key("c")]);
        assert!(is_multisig(&pol));
    }

    #[test]
    fn is_multisig_rejects_timelocked_threshold() {
        let pol = Policy::Thresh(2, vec![key("a"), Policy::Older(100)]);
        assert!(!is_multisig(&pol));
    }
}
