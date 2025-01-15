## Rust implementation of BTC

### How to use:

- To generate transaction: `cargo run --bin tx_gen <filename>`
- To print transaction: `cargo run --bin tx_print <generated_file>`
- To generate block: `cargo run --bin block_gen <filename>`
- To print block: `cargo run --bin block_print <filename>`
- To generate keys: `cargo run --bin key_gen ./miner/<keyname> `
- To mine using generated keys: ` cargo run --bin miner localhost::9000 ./miner/alice.pub.pem`
