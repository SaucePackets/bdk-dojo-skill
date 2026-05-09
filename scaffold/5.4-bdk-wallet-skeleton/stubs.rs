/// scaffold-only pseudocode: do not paste real seeds or use mainnet funds.

pub const SAFE_NETWORKS: &[&str] = &["regtest", "signet"];

pub fn bdk_wallet_skeleton_steps() -> Vec<&'static str> {
    vec![
        "choose regtest or signet",
        "use placeholder descriptors",
        "construct wallet with BDK APIs from current docs",
        "sync against a safe chain source",
        "inspect balance without signing real funds",
    ]
}
