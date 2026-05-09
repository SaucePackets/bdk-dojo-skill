#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BdkBridgeNote {
    pub toy_concept: &'static str,
    pub bdk_concept: &'static str,
    pub what_toy_hides: &'static str,
}

pub fn balance_bridge_note() -> BdkBridgeNote {
    BdkBridgeNote {
        toy_concept: "BalanceSummary classifies toy UTXO value buckets.",
        bdk_concept: "BDK exposes wallet balance categories derived from indexed chain data.",
        what_toy_hides: "Descriptors, script pubkeys, local chain checkpoints, transaction graph policy, and persistence.",
    }
}

pub fn sync_bridge_note() -> BdkBridgeNote {
    BdkBridgeNote {
        toy_concept: "SyncEvent mutates simple wallet state.",
        bdk_concept: "BDK sync/full-scan updates wallet data from chain sources and persists checkpoints.",
        what_toy_hides: "Real chain source APIs, script scanning, transaction anchors, and reorg-safe persistence.",
    }
}
