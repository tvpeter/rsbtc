use std::{env, process::exit};

use btc_lib::{
    crypto::PrivateKey,
    types::{Transaction, TransactionOutput},
    util::Saveable,
    INITIAL_REWARD,
};
use uuid::Uuid;

fn main() {
    let path = if let Some(arg) = env::args().nth(1) {
        arg
    } else {
        eprintln!("Usage: tx_print <tx_file>");
        exit(1);
    };

    let private_key = PrivateKey::new_key();

    let transaction = Transaction::new(
        vec![],
        vec![TransactionOutput {
            unique_id: Uuid::new_v4(),
            value: INITIAL_REWARD * 10u64.pow(8),
            pubkey: private_key.public_key(),
        }],
    );
    transaction
        .save_to_file(path)
        .expect("failed to save transaction");
}
