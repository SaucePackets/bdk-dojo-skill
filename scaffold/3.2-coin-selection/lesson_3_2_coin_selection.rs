// lesson_3_2_coin_selection.rs — Lesson 3.2: Coin Selection
//
// This file contains pre-written tests for the `select_coins` stub.
// Run them with:
//
//   cargo test
//
// All tests should FAIL until you implement `select_coins` in `src/coin_selection.rs`.
// Once your implementation is correct every test in this file will pass.

// This course standardizes on package `bdk-dojo`, imported as `bdk_dojo`.
use bdk_dojo::{select_coins, Amount, FeeRate, OutPoint, Utxo, WalletError};

#[cfg(test)]
mod tests {
    use super::*;

    fn make_utxo(value: u64, vout: u32, seen_at_height: Option<u32>) -> Utxo {
        Utxo {
            outpoint: OutPoint {
                txid: format!("{:064x}", vout),
                vout,
            },
            value: Amount::from_sats(value),
            confirmed: seen_at_height.is_some(),
            spendable: true,
            seen_at_height,
            coinbase: false,
            locked_until: None,
            owned: true,
        }
    }

    #[test]
    fn coin_selection_picks_enough_spendable_utxos() {
        let utxos = vec![
            make_utxo(100_000, 0, Some(750_000)),
            make_utxo(50_000, 1, Some(750_000)),
            make_utxo(30_000, 2, Some(750_000)),
        ];
        let target = 80_000_u64;
        let fee_rate = FeeRate { sat_per_vb: 2 };
        let tip_height = 800_000_u32;

        let result = select_coins(target, fee_rate, &utxos, tip_height);

        assert!(result.is_ok(), "expected Ok");

        let selected = result.unwrap();
        assert!(!selected.selected.is_empty());

        let selected_total: u64 = selected.selected.iter().map(|u| u.value.to_sats()).sum();
        assert!(
            selected_total >= selected.target + selected.estimated_fee,
            "selected total ({selected_total}) must cover target plus fee ({})",
            selected.target + selected.estimated_fee,
        );
        assert_eq!(selected.total_selected, selected_total);

        for u in &selected.selected {
            assert!(
                utxos.iter().any(|orig| orig.outpoint == u.outpoint),
                "selected UTXO {:?} was not in the original pool",
                u.outpoint
            );
        }
    }

    #[test]
    fn coin_selection_respects_chain_spendability_policy() {
        let mut immature_coinbase = make_utxo(100_000, 0, Some(799_950));
        immature_coinbase.coinbase = true;

        let mut locked = make_utxo(100_000, 1, Some(799_999));
        locked.locked_until = Some(800_010);

        let mut foreign = make_utxo(100_000, 2, Some(799_999));
        foreign.owned = false;

        let spendable = make_utxo(100_000, 3, Some(799_999));
        let utxos = vec![immature_coinbase, locked, foreign, spendable.clone()];

        let selected = select_coins(80_000, FeeRate { sat_per_vb: 1 }, &utxos, 800_000).unwrap();

        assert_eq!(selected.selected, vec![spendable]);
    }

    #[test]
    fn coin_selection_reports_insufficient_funds() {
        let utxos = vec![make_utxo(10_000, 0, Some(750_000))];
        let target = 80_000_u64;
        let fee_rate = FeeRate { sat_per_vb: 2 };
        let tip_height = 800_000_u32;

        let result = select_coins(target, fee_rate, &utxos, tip_height);

        match result {
            Err(WalletError::InsufficientFunds { needed, available }) => {
                assert!(
                    available < needed,
                    "available ({available}) should be less than needed ({needed})"
                );
            }
            other => panic!("expected WalletError::InsufficientFunds, got: {other:?}"),
        }
    }
}
