use crate::utxo::Utxo;

pub const COINBASE_MATURITY: u32 = 100;

pub fn confirmations(utxo: &Utxo, tip_height: u32) -> u32 {
    match utxo.seen_at_height {
        Some(height) if height <= tip_height => tip_height - height + 1,
        _ => 0,
    }
}

pub fn is_spendable(utxo: &Utxo, tip_height: u32) -> bool {
    if !utxo.owned || !utxo.spendable {
        return false;
    }

    if let Some(lock_height) = utxo.locked_until {
        if tip_height < lock_height {
            return false;
        }
    }

    if utxo.coinbase && confirmations(utxo, tip_height) < COINBASE_MATURITY {
        return false;
    }

    true
}
