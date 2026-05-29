/// src/wallet.rs

use crate::utxo::{OutPoint, Utxo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyncEvent {
    Found(Utxo),
    Confirmed { outpoint: OutPoint, height: u32 },
    Spent(OutPoint),
    Reorged { outpoint: OutPoint },
    TipAdvanced(u32),
}

impl WalletState {
    pub fn apply(&mut self, event: SyncEvent) {
        todo!("mutate wallet state for each sync event")
    }
}
