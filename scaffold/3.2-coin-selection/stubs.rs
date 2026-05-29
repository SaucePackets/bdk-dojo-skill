/// src/coin_selection.rs

use crate::{FeeRate, Utxo, WalletError};

pub struct CoinSelection {
    pub selected: Vec<Utxo>,
    pub target: u64,
    pub estimated_fee: u64,
    pub total_selected: u64,
}

pub fn select_coins(target: u64, fee_rate: FeeRate, utxos: &[Utxo], tip_height: u32) -> Result<CoinSelection, WalletError> {
    todo!("select enough spendable wallet UTXOs for target plus estimated fee")
}
