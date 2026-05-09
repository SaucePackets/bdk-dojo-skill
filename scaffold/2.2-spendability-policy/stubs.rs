/// src/chain.rs

use crate::utxo::Utxo;

pub const COINBASE_MATURITY: u32 = 100;

pub fn is_spendable(utxo: &Utxo, tip_height: u32) -> bool {
    todo!("combine ownership, lock height, and coinbase maturity")
}
