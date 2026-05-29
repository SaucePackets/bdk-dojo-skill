/// src/change.rs

pub const DUST_LIMIT: u64 = 546;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeDecision {
    NoChange,
    Change(u64),
    AddToFee(u64),
}

pub fn decide_change(leftover: u64) -> ChangeDecision {
    todo!("classify leftover sats as no change, change output, or extra fee")
}
