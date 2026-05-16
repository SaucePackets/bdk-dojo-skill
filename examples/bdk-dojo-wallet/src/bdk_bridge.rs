#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BdkBridgeNote {
    pub toy_concept: &'static str,
    pub bdk_concept: &'static str,
    pub what_toy_hides: &'static str,
}

pub fn balance_bridge_note() -> BdkBridgeNote {
    BdkBridgeNote {
        toy_concept: "BalanceSummary classifies toy UTXO value buckets.",
        bdk_concept: "bdk_chain::Balance exposes immature, trusted_pending, untrusted_pending, and confirmed categories derived from indexed chain data.",
        what_toy_hides: "Descriptors, script pubkeys, immature coinbase value, local chain checkpoints, transaction graph policy, and persistence.",
    }
}

pub fn sync_bridge_note() -> BdkBridgeNote {
    BdkBridgeNote {
        toy_concept: "SyncEvent mutates simple wallet state.",
        bdk_concept: "BDK full_scan/sync updates wallet data from Esplora, Electrum, or bitcoind RPC chain sources and persists checkpoints.",
        what_toy_hides: "Real chain source APIs, script scanning, transaction anchors, and reorg-safe persistence.",
    }
}

pub fn miniscript_bridge_note() -> BdkBridgeNote {
    BdkBridgeNote {
        toy_concept: "Policy enum with Key, Older, After, And, Or, Thresh, HashLock",
        bdk_concept: "miniscript::Miniscript<Pk> with typed fragments, satisfaction costs, and malleability analysis",
        what_toy_hides: "Script compilation, witness stack construction, type system constraints, resource limits, key hashing",
    }
}
