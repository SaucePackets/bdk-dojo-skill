pub const DUST_LIMIT: u64 = 546;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeDecision {
    NoChange,
    Change(u64),
    AddToFee(u64),
}

pub fn decide_change(leftover: u64) -> ChangeDecision {
    if leftover == 0 {
        ChangeDecision::NoChange
    } else if leftover < DUST_LIMIT {
        ChangeDecision::AddToFee(leftover)
    } else {
        ChangeDecision::Change(leftover)
    }
}
