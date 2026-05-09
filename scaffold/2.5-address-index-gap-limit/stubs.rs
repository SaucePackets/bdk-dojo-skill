/// src/wallet.rs

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddressRecord {
    pub index: u32,
    pub address: String,
    pub used: bool,
}

impl WalletState {
    pub fn next_unused_address(&mut self) -> AddressRecord {
        todo!("return existing unused address or derive the next toy address")
    }
}
