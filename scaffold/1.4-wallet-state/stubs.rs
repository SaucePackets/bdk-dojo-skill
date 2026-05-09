/// src/wallet.rs

use crate::balance::{BalanceSummary, classify_balance};
use crate::utxo::Utxo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletState {
    pub utxos: Vec<Utxo>,
    pub tip_height: u32,
}

impl WalletState {
    pub fn new(tip_height: u32) -> Self {
        todo!("create an empty wallet state at the current tip height")
    }

    pub fn balance(&self) -> BalanceSummary {
        todo!("delegate to classify_balance")
    }
}
