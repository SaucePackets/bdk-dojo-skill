/// src/errors.rs

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WalletError {
    InsufficientFunds { needed: u64, available: u64 },
    DustChange { change: u64, dust_limit: u64 },
    UnsafePsbt(String),
    InvalidDescriptor,
    UnknownRecipient(String),
}
