/// src/spending_satisfaction.rs
///
/// Analyze what it takes to satisfy a spending policy.
use crate::miniscript_ast::Policy;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SatisfactionSummary {
    /// Minimum number of distinct signatures required
    pub signatures_required: usize,
    /// Highest timelock height that must pass (None if no timelock)
    pub timelock_required: Option<u32>,
    /// Whether a hash preimage must be revealed
    pub preimage_required: bool,
    /// Number of alternative satisfaction paths (from Or branches)
    pub or_branches_count: usize,
}

/// Analyze a policy and return what's needed to satisfy it.
pub fn analyze_satisfaction(policy: &Policy) -> SatisfactionSummary {
    let signatures_required = count_sigs(policy);
    let timelock_required = max_timelock(policy);
    let preimage_required = has_preimage(policy);
    let or_branches_count = count_or_branches(policy);

    SatisfactionSummary {
        signatures_required,
        timelock_required,
        preimage_required,
        or_branches_count,
    }
}

fn count_sigs(policy: &Policy) -> usize {
    match policy {
        Policy::Key(_) => 1,
        Policy::Older(_) | Policy::After(_) => 0,
        Policy::HashLock(_) => 0,
        Policy::And(children) => children.iter().map(count_sigs).sum(),
        Policy::Or(children) => children.iter().map(count_sigs).min().unwrap_or(0),
        Policy::Thresh(k, children) => {
            let mut sigs: Vec<usize> = children.iter().map(count_sigs).collect();
            sigs.sort();
            sigs.iter().take(*k as usize).sum()
        }
    }
}

fn max_timelock(policy: &Policy) -> Option<u32> {
    match policy {
        Policy::Older(n) | Policy::After(n) => Some(*n),
        Policy::Key(_) | Policy::HashLock(_) => None,
        Policy::And(children) => children.iter().filter_map(max_timelock).max(),
        Policy::Or(children) => children
            .iter()
            .filter_map(max_timelock)
            .min()
            .or_else(|| children.iter().filter_map(max_timelock).max()),
        Policy::Thresh(_, children) => children.iter().filter_map(max_timelock).max(),
    }
}

fn has_preimage(policy: &Policy) -> bool {
    match policy {
        Policy::HashLock(_) => true,
        Policy::Key(_) | Policy::Older(_) | Policy::After(_) => false,
        Policy::And(children) | Policy::Or(children) => children.iter().any(|c| has_preimage(c)),
        Policy::Thresh(_, children) => children.iter().any(|c| has_preimage(c)),
    }
}

fn count_or_branches(policy: &Policy) -> usize {
    match policy {
        Policy::Or(children) => children.len(),
        Policy::And(children) | Policy::Thresh(_, children) => {
            children.iter().map(|c| count_or_branches(c)).sum()
        }
        _ => 0,
    }
}

/// Check whether the policy can be satisfied with the given preconditions.
pub fn can_satisfy_with(
    policy: &Policy,
    available_signatures: usize,
    current_height: u32,
    has_preimage: bool,
) -> bool {
    let summary = analyze_satisfaction(policy);

    if available_signatures < summary.signatures_required {
        return false;
    }
    if let Some(timelock) = summary.timelock_required {
        if current_height < timelock {
            return false;
        }
    }
    if summary.preimage_required && !has_preimage {
        return false;
    }

    true
}

/// Describe in plain English how to satisfy this policy.
pub fn describe_satisfaction(policy: &Policy) -> String {
    let summary = analyze_satisfaction(policy);
    let mut parts: Vec<String> = Vec::new();

    if summary.signatures_required == 1 {
        parts.push("1 signature".to_string());
    } else if summary.signatures_required > 1 {
        parts.push(format!("{} signatures", summary.signatures_required));
    }

    if let Some(height) = summary.timelock_required {
        parts.push(format!("wait until block {height}"));
    }

    if summary.preimage_required {
        parts.push("reveal a hash preimage".to_string());
    }

    if summary.or_branches_count > 0 {
        parts.push(format!(
            "choose from {} alternative path(s)",
            summary.or_branches_count
        ));
    }

    if parts.is_empty() {
        return "No special conditions required.".to_string();
    }

    let capitalized = format!("{}{}", parts[0][..1].to_uppercase(), &parts[0][1..]);
    parts[0] = capitalized;
    format!("Provide {} to spend.", parts.join(", "))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(name: &str) -> Policy {
        Policy::Key(name.to_string())
    }

    #[test]
    fn single_key_requires_one_signature_no_timelock() {
        let summary = analyze_satisfaction(&key("alice"));
        assert_eq!(summary.signatures_required, 1);
        assert_eq!(summary.timelock_required, None);
        assert!(!summary.preimage_required);
    }

    #[test]
    fn timelocked_policy_requires_height_and_signature() {
        let pol = Policy::And(vec![key("alice"), Policy::Older(100)]);
        let summary = analyze_satisfaction(&pol);
        assert_eq!(summary.signatures_required, 1);
        assert_eq!(summary.timelock_required, Some(100));
    }

    #[test]
    fn multisig_2_of_3_requires_two_signatures() {
        let pol = Policy::Thresh(2, vec![key("a"), key("b"), key("c")]);
        let summary = analyze_satisfaction(&pol);
        assert_eq!(summary.signatures_required, 2);
    }

    #[test]
    fn or_policy_can_be_satisfied_via_either_branch() {
        let pol = Policy::Or(vec![key("alice"), key("bob")]);
        let summary = analyze_satisfaction(&pol);
        assert_eq!(summary.signatures_required, 1);
        assert_eq!(summary.or_branches_count, 2);
    }

    #[test]
    fn can_satisfy_with_checks_all_conditions() {
        let pol = Policy::And(vec![
            Policy::Thresh(2, vec![key("a"), key("b"), key("c")]),
            Policy::Older(100),
        ]);

        // Not enough sigs
        assert!(!can_satisfy_with(&pol, 1, 200, false));
        // Timelock not met
        assert!(!can_satisfy_with(&pol, 2, 50, false));
        // All conditions met
        assert!(can_satisfy_with(&pol, 2, 200, false));
    }

    #[test]
    fn describe_satisfaction_is_readable() {
        let pol = Policy::Thresh(2, vec![key("a"), key("b"), key("c")]);
        let desc = describe_satisfaction(&pol);
        assert!(desc.contains("2 signatures"));
    }
}
