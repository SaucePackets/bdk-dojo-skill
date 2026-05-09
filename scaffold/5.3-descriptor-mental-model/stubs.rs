/// src/descriptors.rs

pub enum DescriptorKind {
    SingleSig,
    MultiSig,
    Unknown,
}

pub fn classify_descriptor(descriptor: &str) -> DescriptorKind {
    todo!("classify toy descriptor policy shape")
}
