use wallet_balance_kata::{summarize_balance, Utxo};

fn main() {
    let utxos = vec![
        Utxo { value: 50_000, confirmed: true, spendable: true },
        Utxo { value: 20_000, confirmed: false, spendable: true },
        Utxo { value: 10_000, confirmed: false, spendable: false },
    ];

    let summary = summarize_balance(&utxos);

    println!("confirmed: {}", summary.confirmed);
    println!("trusted_pending: {}", summary.trusted_pending);
    println!("untrusted_pending: {}", summary.untrusted_pending);
    println!("total_spendable: {}", summary.total_spendable);
}
