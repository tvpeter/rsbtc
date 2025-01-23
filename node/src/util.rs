use anyhow::Result;
use btc_lib::{types::Blockchain, util::Saveable};

pub async fn load_blockchain(blockchain_file: &str) -> Result<()> {
    println!("blockchain file exists, loading...");

    let new_blockchain = Blockchain::load_from_file(&blockchain_file)?;
    println!("blockchain loaded");
    let mut blockchain = crate::BLOCKCHAIN.write().await;
    *blockchain = new_blockchain;
    println!("rebuilding utxos...");
    blockchain.rebuild_utxos();
    println!("utxos rebuilt");
    println!("checking if target needs to be adjusted...");
    println!("current target {}", blockchain.target());
    blockchain.try_adjust_target();
    println!("new target: {}", blockchain.target());
    println!("initialization complete");
    Ok(())
}
