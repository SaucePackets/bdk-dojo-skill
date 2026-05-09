/// src/psbt_review.rs

use crate::{TxPlan, WalletError};

pub struct WalletPolicy {
    pub allowed_recipients: Vec<String>,
    pub max_fee: u64,
}

pub struct PsbtReview {
    pub approved: bool,
    pub warnings: Vec<String>,
}

pub fn review_plan(plan: &TxPlan, policy: &WalletPolicy) -> Result<PsbtReview, WalletError> {
    todo!("reject unknown recipients and excessive fees before signing")
}
