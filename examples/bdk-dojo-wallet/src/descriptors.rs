use crate::errors::WalletError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DescriptorKind {
    SingleSig,
    MultiSig,
    Unknown,
}

pub fn classify_descriptor(descriptor: &str) -> DescriptorKind {
    if descriptor.contains("multi(") || descriptor.contains("sortedmulti(") {
        DescriptorKind::MultiSig
    } else if descriptor.contains("wpkh(")
        || descriptor.contains("pkh(")
        || descriptor.contains("tr(")
    {
        DescriptorKind::SingleSig
    } else {
        DescriptorKind::Unknown
    }
}

pub fn validate_toy_descriptor(descriptor: &str) -> Result<DescriptorKind, WalletError> {
    match classify_descriptor(descriptor) {
        DescriptorKind::Unknown => Err(WalletError::InvalidDescriptor),
        kind => Ok(kind),
    }
}
