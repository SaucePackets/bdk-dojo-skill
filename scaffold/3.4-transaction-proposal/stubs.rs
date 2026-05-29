/// src/tx_plan.rs

use crate::{ChangeDecision, FeeRate, Utxo, WalletError};

pub struct TxPlan {
    pub selected: Vec<Utxo>,
    pub recipient: String,
    pub amount: u64,
    pub fee: u64,
    pub change: ChangeDecision,
}

pub fn propose_transaction(recipient: String, amount: u64, fee_rate: FeeRate, utxos: &[Utxo], tip_height: u32) -> Result<TxPlan, WalletError> {
    todo!("select coins and build an unsigned transaction plan")
}
