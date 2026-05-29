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
    todo!("return 'single key', 'timelocked', 'multisig', 'hash locked', or 'composite'")
}

/// Count all keys referenced anywhere in the policy tree.
pub fn count_keys(policy: &Policy) -> usize {
    todo!("walk the tree and count Key nodes")
}

/// Returns true if any branch has an Older or After timelock.
pub fn is_timelocked(policy: &Policy) -> bool {
    todo!("walk the tree checking for Older or After nodes")
}

/// Returns true if the policy is a Thresh with multiple keys and no other types.
pub fn is_multisig(policy: &Policy) -> bool {
    todo!("check if thresh(k, keys...) with no timelocks/hashlocks")
}
