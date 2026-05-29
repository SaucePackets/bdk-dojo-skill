use bdk_dojo_wallet::{Amount, OutPoint, Utxo, classify_balance};

fn main() {
    let utxos = vec![
        Utxo::new(
            OutPoint {
                txid: "00".repeat(32),
                vout: 0,
            },
            Amount::from_sats(50_000),
        )
        .confirmed_at(800_000),
        Utxo::new(
            OutPoint {
                txid: "11".repeat(32),
                vout: 1,
            },
            Amount::from_sats(20_000),
        ),
        Utxo::new(
            OutPoint {
                txid: "22".repeat(32),
                vout: 2,
            },
            Amount::from_sats(10_000),
        )
        .unspendable(),
    ];

    let balance = classify_balance(&utxos);

    println!("confirmed: {}", balance.confirmed);
    println!("trusted_pending: {}", balance.trusted_pending);
    println!("untrusted_pending: {}", balance.untrusted_pending);
    println!("total_spendable: {}", balance.total_spendable);
}
