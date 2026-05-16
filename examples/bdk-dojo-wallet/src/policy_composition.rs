/// src/policy_composition.rs
///
/// Compose and analyze real-world spending policies.

use crate::miniscript_ast::Policy;

/// Build a composed policy by name with the given parameters.
///
/// Known patterns:
/// - "multisig_with_recovery": thresh(1, or(thresh(k, keys...), and(recovery, older(t))))
/// - "escrow": thresh(2, or(and(agent, buyer), and(agent, seller), and(buyer, seller), arbitrator))
pub fn compose_policy(pattern: &str, params: &[&str]) -> Policy {
    match pattern {
        "multisig_with_recovery" => {
            let num_keys: usize = params[0].parse().unwrap_or(2);
            let threshold: u32 = params[1].parse().unwrap_or(2);
            let timelock: u32 = params[2].parse().unwrap_or(1000);
            let recovery_key = params[3];

            let keys: Vec<Policy> = (0..num_keys)
                .map(|i| Policy::Key(format!("key_{i}")))
                .collect();
            let multisig = Policy::Thresh(threshold, keys);
            let recovery = Policy::And(vec![
                Policy::Key(recovery_key.to_string()),
                Policy::Older(timelock),
            ]);

            Policy::Thresh(1, vec![multisig, recovery])
        }
        "escrow" => {
            let agent = params.get(0).copied().unwrap_or("agent");
            let buyer = params.get(1).copied().unwrap_or("buyer");
            let seller = params.get(2).copied().unwrap_or("seller");
            let arbitrator = params.get(3).copied().unwrap_or("arbitrator");

            Policy::Thresh(
                2,
                vec![
                    Policy::Or(vec![
                        Policy::And(vec![
                            Policy::Key(agent.to_string()),
                            Policy::Key(buyer.to_string()),
                        ]),
                        Policy::And(vec![
                            Policy::Key(agent.to_string()),
                            Policy::Key(seller.to_string()),
                        ]),
                        Policy::And(vec![
                            Policy::Key(buyer.to_string()),
                            Policy::Key(seller.to_string()),
                        ]),
                    ]),
                    Policy::Key(arbitrator.to_string()),
                ],
            )
        }
        _ => Policy::Key("unknown".to_string()),
    }
}

/// Return a human-readable summary of what a policy allows.
pub fn describe_policy(policy: &Policy) -> String {
    match policy {
        Policy::Key(name) => format!("single key: {name}"),
        Policy::Older(n) => format!("timelock: wait {n} blocks"),
        Policy::After(n) => format!("timelock: wait until block {n}"),
        Policy::And(children) => {
            let parts: Vec<String> = children.iter().map(describe_policy).collect();
            format!("all of ({})", parts.join(", "))
        }
        Policy::Or(children) => {
            let parts: Vec<String> = children.iter().map(describe_policy).collect();
            format!("any of ({})", parts.join(", "))
        }
        Policy::Thresh(k, children) => {
            if children.iter().all(|c| matches!(c, Policy::Key(_))) {
                let n = children.len();
                format!("{k}-of-{n} multisig")
            } else {
                let parts: Vec<String> = children.iter().map(describe_policy).collect();
                format!("{k}-of-threshold({})", parts.join(", "))
            }
        }
        Policy::HashLock(_) => "hash locked: preimage required".to_string(),
    }
}

/// Return all distinct timelock values found in the policy tree.
pub fn extract_timelocks(policy: &Policy) -> Vec<u32> {
    let mut values: Vec<u32> = Vec::new();
    collect_timelocks(policy, &mut values);
    values.sort();
    values.dedup();
    values
}

fn collect_timelocks(policy: &Policy, values: &mut Vec<u32>) {
    match policy {
        Policy::Older(n) => values.push(*n),
        Policy::After(n) => values.push(*n),
        Policy::Key(_) | Policy::HashLock(_) => {}
        Policy::And(children) | Policy::Or(children) => {
            for child in children {
                collect_timelocks(child, values);
            }
        }
        Policy::Thresh(_, children) => {
            for child in children {
                collect_timelocks(child, values);
            }
        }
    }
}

/// Return the minimum number of signatures needed to satisfy the policy.
pub fn required_signatures(policy: &Policy) -> usize {
    match policy {
        Policy::Key(_) => 1,
        Policy::Older(_) | Policy::After(_) => 0,
        Policy::HashLock(_) => 0,
        Policy::And(children) => children.iter().map(required_signatures).sum(),
        Policy::Or(children) => children.iter().map(required_signatures).min().unwrap_or(0),
        Policy::Thresh(k, children) => {
            let mut sigs: Vec<usize> = children.iter().map(required_signatures).collect();
            sigs.sort();
            sigs.iter().take(*k as usize).sum()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(name: &str) -> Policy {
        Policy::Key(name.to_string())
    }

    fn is_timelocked(policy: &Policy) -> bool {
        match policy {
            Policy::Older(_) | Policy::After(_) => true,
            Policy::Key(_) | Policy::HashLock(_) => false,
            Policy::And(c) | Policy::Or(c) => c.iter().any(is_timelocked),
            Policy::Thresh(_, c) => c.iter().any(is_timelocked),
        }
    }

    #[test]
    fn compose_multisig_with_recovery_produces_correct_tree() {
        let pol = compose_policy("multisig_with_recovery", &["3", "2", "1000", "recovery"]);
        assert_eq!(required_signatures(&pol), 1);
        assert!(is_timelocked(&pol));
    }

    #[test]
    fn describe_policy_returns_human_readable_summary_for_known_patterns() {
        assert_eq!(describe_policy(&key("alice")), "single key: alice");
        assert_eq!(
            describe_policy(&Policy::Thresh(2, vec![key("a"), key("b"), key("c")])),
            "2-of-3 multisig"
        );
        assert_eq!(
            describe_policy(&Policy::Older(100)),
            "timelock: wait 100 blocks"
        );
    }

    #[test]
    fn extract_timelocks_finds_all_distinct_values() {
        let pol = Policy::And(vec![
            Policy::Older(100),
            Policy::Or(vec![Policy::Older(200), Policy::Older(100)]),
        ]);
        assert_eq!(extract_timelocks(&pol), vec![100, 200]);
    }

    #[test]
    fn multisig_2_of_3_requires_2_signatures() {
        let pol = Policy::Thresh(2, vec![key("a"), key("b"), key("c")]);
        assert_eq!(required_signatures(&pol), 2);
    }

    #[test]
    fn and_policy_requires_sum_of_signatures() {
        let pol = Policy::And(vec![
            Policy::Thresh(2, vec![key("a"), key("b"), key("c")]),
            key("d"),
        ]);
        assert_eq!(required_signatures(&pol), 3);
    }
}
