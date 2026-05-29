use crate::errors::WalletError;
use crate::tx_plan::TxPlan;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletPolicy {
    pub allowed_recipients: Vec<String>,
    pub max_fee: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PsbtReview {
    pub approved: bool,
    pub warnings: Vec<String>,
}

pub fn review_plan(plan: &TxPlan, policy: &WalletPolicy) -> Result<PsbtReview, WalletError> {
    if !policy.allowed_recipients.contains(&plan.recipient) {
        return Err(WalletError::UnknownRecipient(plan.recipient.clone()));
    }

    if plan.fee > policy.max_fee {
        return Err(WalletError::UnsafePsbt(format!(
            "fee {} exceeds max {}",
            plan.fee, policy.max_fee
        )));
    }

    Ok(PsbtReview {
        approved: true,
        warnings: Vec::new(),
    })
}
