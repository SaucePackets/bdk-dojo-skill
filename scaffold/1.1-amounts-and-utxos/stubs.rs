/// src/amount.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Amount {
    sats: u64,
}

impl Amount {
    pub fn from_sats(sats: u64) -> Self {
        todo!("store sats exactly")
    }

    pub fn to_sats(self) -> u64 {
        todo!("return sats exactly")
    }
}

/// src/utxo.rs

use crate::amount::Amount;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutPoint {
    pub txid: String,
    pub vout: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Utxo {
    pub outpoint: OutPoint,
    pub value: Amount,
}
