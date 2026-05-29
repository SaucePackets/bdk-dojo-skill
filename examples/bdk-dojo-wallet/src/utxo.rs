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
    pub confirmed: bool,
    pub spendable: bool,
    pub seen_at_height: Option<u32>,
    pub coinbase: bool,
    pub locked_until: Option<u32>,
    pub owned: bool,
}

impl Utxo {
    pub fn new(outpoint: OutPoint, value: Amount) -> Self {
        Self {
            outpoint,
            value,
            confirmed: false,
            spendable: true,
            seen_at_height: None,
            coinbase: false,
            locked_until: None,
            owned: true,
        }
    }

    pub fn confirmed_at(mut self, height: u32) -> Self {
        self.confirmed = true;
        self.seen_at_height = Some(height);
        self
    }

    pub fn unspendable(mut self) -> Self {
        self.spendable = false;
        self
    }

    pub fn coinbase(mut self) -> Self {
        self.coinbase = true;
        self
    }

    pub fn locked_until(mut self, height: u32) -> Self {
        self.locked_until = Some(height);
        self
    }

    pub fn foreign(mut self) -> Self {
        self.owned = false;
        self
    }
}
