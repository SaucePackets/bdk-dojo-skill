use bdk_dojo_wallet::{Amount, OutPoint, Utxo, classify_balance};

fn main() {
    let utxos = vec![
        Utxo {
            outpoint: OutPoint {
                txid: "00".repeat(32),
                vout: 0,
            },
            value: Amount::from_sats(50_000),
            confirmed: true,
            spendable: true,
        },
        Utxo {
            outpoint: OutPoint {
                txid: "11".repeat(32),
                vout: 1,
            },
            value: Amount::from_sats(20_000),
            confirmed: false,
            spendable: true,
        },
        Utxo {
            outpoint: OutPoint {
                txid: "22".repeat(32),
                vout: 2,
            },
            value: Amount::from_sats(10_000),
            confirmed: false,
            spendable: false,
        },
    ];

    let balance = classify_balance(&utxos);

    println!("confirmed: {}", balance.confirmed);
    println!("trusted_pending: {}", balance.trusted_pending);
    println!("untrusted_pending: {}", balance.untrusted_pending);
    println!("total_spendable: {}", balance.total_spendable);
}
