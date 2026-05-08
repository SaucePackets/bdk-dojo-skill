/// src/balance.rs

use crate::utxo::Utxo;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BalanceSummary {
    pub confirmed: u64,
    pub trusted_pending: u64,
    pub untrusted_pending: u64,
    pub total_spendable: u64,
}

pub fn classify_balance(utxos: &[Utxo]) -> BalanceSummary {
    todo!("classify UTXOs by confirmation and spendability")
}
