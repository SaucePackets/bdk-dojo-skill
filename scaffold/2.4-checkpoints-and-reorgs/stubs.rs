/// src/wallet.rs

impl WalletState {
    pub fn rollback_to_height(&mut self, height: u32) {
        todo!("rollback checkpoints and unconfirm UTXOs above the new tip")
    }
}
