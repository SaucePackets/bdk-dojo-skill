/// src/descriptors.rs

use crate::WalletError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DescriptorKind {
    SingleSig,
    MultiSig,
    Unknown,
}

pub fn classify_descriptor(descriptor: &str) -> DescriptorKind {
    todo!("classify toy descriptor policy shape")
}

pub fn validate_toy_descriptor(descriptor: &str) -> Result<DescriptorKind, WalletError> {
    todo!("reject unknown toy descriptor shapes")
}
