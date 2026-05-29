use crate::change::{ChangeDecision, decide_change};
use crate::coin_selection::select_coins;
use crate::errors::WalletError;
use crate::fees::FeeRate;
use crate::utxo::Utxo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TxPlan {
    pub selected: Vec<Utxo>,
    pub recipient: String,
    pub amount: u64,
    pub fee: u64,
    pub change: ChangeDecision,
}

pub fn propose_transaction(
    recipient: String,
    amount: u64,
    fee_rate: FeeRate,
    utxos: &[Utxo],
    tip_height: u32,
) -> Result<TxPlan, WalletError> {
    let selection = select_coins(amount, fee_rate, utxos, tip_height)?;
    let leftover = selection.total_selected - amount - selection.estimated_fee;
    let change = decide_change(leftover);
    let fee = match change {
        ChangeDecision::AddToFee(extra) => selection.estimated_fee + extra,
        _ => selection.estimated_fee,
    };

    Ok(TxPlan {
        selected: selection.selected,
        recipient,
        amount,
        fee,
        change,
    })
}
