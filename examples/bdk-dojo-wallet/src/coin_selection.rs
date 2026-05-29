use crate::chain::is_spendable;
use crate::errors::WalletError;
use crate::fees::{FeeRate, TxSizeEstimate, fee};
use crate::utxo::Utxo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoinSelection {
    pub selected: Vec<Utxo>,
    pub target: u64,
    pub estimated_fee: u64,
    pub total_selected: u64,
}

pub fn estimate_tx_size(input_count: usize, output_count: usize) -> TxSizeEstimate {
    TxSizeEstimate::new(10 + (input_count as u64 * 68) + (output_count as u64 * 31))
}

pub fn select_coins(
    target: u64,
    fee_rate: FeeRate,
    utxos: &[Utxo],
    tip_height: u32,
) -> Result<CoinSelection, WalletError> {
    let mut selected = Vec::new();
    let mut total_selected = 0;
    let mut estimated_fee = fee(estimate_tx_size(0, 2), fee_rate);

    for utxo in utxos.iter().filter(|utxo| is_spendable(utxo, tip_height)) {
        selected.push(utxo.clone());
        total_selected += utxo.value.to_sats();
        estimated_fee = fee(estimate_tx_size(selected.len(), 2), fee_rate);

        if total_selected >= target + estimated_fee {
            return Ok(CoinSelection {
                selected,
                target,
                estimated_fee,
                total_selected,
            });
        }
    }

    Err(WalletError::InsufficientFunds {
        needed: target + estimated_fee,
        available: total_selected,
    })
}
