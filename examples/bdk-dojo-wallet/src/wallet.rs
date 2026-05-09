use crate::balance::{BalanceSummary, classify_balance};
use crate::utxo::{OutPoint, Utxo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddressRecord {
    pub index: u32,
    pub address: String,
    pub used: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyncEvent {
    Found(Utxo),
    Confirmed { outpoint: OutPoint, height: u32 },
    Spent(OutPoint),
    Reorged { outpoint: OutPoint },
    TipAdvanced(u32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletState {
    pub utxos: Vec<Utxo>,
    pub tip_height: u32,
    pub checkpoints: Vec<u32>,
    pub addresses: Vec<AddressRecord>,
}

impl WalletState {
    pub fn new(tip_height: u32) -> Self {
        Self {
            utxos: Vec::new(),
            tip_height,
            checkpoints: vec![tip_height],
            addresses: Vec::new(),
        }
    }

    pub fn balance(&self) -> BalanceSummary {
        classify_balance(&self.utxos)
    }

    pub fn apply(&mut self, event: SyncEvent) {
        match event {
            SyncEvent::Found(utxo) => {
                if !self
                    .utxos
                    .iter()
                    .any(|existing| existing.outpoint == utxo.outpoint)
                {
                    self.utxos.push(utxo);
                }
            }
            SyncEvent::Confirmed { outpoint, height } => {
                if let Some(utxo) = self.utxos.iter_mut().find(|utxo| utxo.outpoint == outpoint) {
                    utxo.confirmed = true;
                    utxo.seen_at_height = Some(height);
                }
            }
            SyncEvent::Spent(outpoint) => {
                self.utxos.retain(|utxo| utxo.outpoint != outpoint);
            }
            SyncEvent::Reorged { outpoint } => {
                if let Some(utxo) = self.utxos.iter_mut().find(|utxo| utxo.outpoint == outpoint) {
                    utxo.confirmed = false;
                    utxo.seen_at_height = None;
                }
            }
            SyncEvent::TipAdvanced(height) => {
                self.tip_height = height;
                if self.checkpoints.last().copied() != Some(height) {
                    self.checkpoints.push(height);
                }
            }
        }
    }

    pub fn rollback_to_height(&mut self, height: u32) {
        self.tip_height = height;
        self.checkpoints.retain(|checkpoint| *checkpoint <= height);
        if self.checkpoints.last().copied() != Some(height) {
            self.checkpoints.push(height);
        }

        for utxo in &mut self.utxos {
            if utxo.seen_at_height.is_some_and(|seen| seen > height) {
                utxo.confirmed = false;
                utxo.seen_at_height = None;
            }
        }
    }

    pub fn next_unused_address(&mut self) -> AddressRecord {
        if let Some(record) = self.addresses.iter().find(|record| !record.used) {
            return record.clone();
        }

        let index = self.addresses.len() as u32;
        let record = AddressRecord {
            index,
            address: format!("bdk-dojo-address-{index}"),
            used: false,
        };
        self.addresses.push(record.clone());
        record
    }

    pub fn mark_address_used(&mut self, index: u32) {
        if let Some(record) = self
            .addresses
            .iter_mut()
            .find(|record| record.index == index)
        {
            record.used = true;
        }
    }
}
