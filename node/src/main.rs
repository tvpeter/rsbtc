use anyhow::{Context, Ok, Result};
use argh::FromArgs;
use btc_lib::network::Message;
use btc_lib::types::Blockchain;
use dashmap::DashMap;
use static_init::dynamic;
use std::path::Path;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;

mod util;

#[dynamic]
pub static BLOCKCHAIN: RwLock<Blockchain> = RwLock::new(Blockchain::new());

//nodes pool
pub static NODES: DashMap<String, TcpStream> = DashMap::new();

#[derive(FromArgs)]
/// A toy blockchain node
struct Args {
    #[argh(option, default = "9000")]
    /// port number
    port: u16,
    #[argh(option, default = "String::from(\"./blockchain.cbor\")")]
    /// blockchain file location
    blockchain_file: String,
    #[argh(positional)]
    /// addresses of initial nodes
    nodes: Vec<String>,
}

pub async fn populate_connections(nodes: &[String]) -> Result<()> {
    println!("trying to connect to other nodes...");

    for node in nodes {
        println!("connecting to node: {}", node);
        let mut stream = TcpStream::connect(&node).await?;

        let message = Message::DiscoverNodes;

        message.send_async(&mut stream).await?;
        println!("send DiscoverNode to {}", node);

        let message = Message::receive_async(&mut stream).await?;

        match message {
            Message::NodeList(child_nodes) => {
                println!("recived node list from {}", node);

                for child_node in child_nodes {
                    println!("adding node {}", &child_node);

                    let new_stream = TcpStream::connect(&child_node).await?;

                    crate::NODES.insert(child_node, new_stream);
                }
            }
            _ => {
                println!("unexpected message from {}", node);
            }
        }
        crate::NODES.insert(node.clone(), stream);
    }
    Ok(())
}

pub async fn find_longest_chain_node() -> Result<(String, u32)> {
    println!("finding nodes with the highest blockchain length..");

    let mut longest_name = String::new();
    let mut longest_count = 0;
    let all_nodes = crate::NODES
        .iter()
        .map(|x| x.key().clone())
        .collect::<Vec<_>>();

    for node in all_nodes {
        println!("asking {} for blockchain length", node);

        let mut stream = crate::NODES.get_mut(&node).context("no node")?;
        let message = Message::AskDifference(0);
        message.send_async(&mut *stream).await.unwrap();
        println!("sent AskDifference to {}", node);
        let message = Message::receive_async(&mut *stream).await?;

        match message {
            Message::Difference(count) => {
                println!("received Difference from {}", node);
                if count > longest_count {
                    println!("new longest blockchain {} blocks from {node}", count);
                    longest_count = count;
                    longest_name = node;
                }
            }
            e => {
                println!("unexpected message from {} : {:?}", node, e);
            }
        }
    }
    Ok((longest_name, longest_count as u32))
}

pub async fn download_blockchain(node: &str, count: u32) -> Result<()> {
    let mut stream = crate::NODES.get_mut(node).unwrap();
    for i in 0..count as usize {
        let message = Message::FetchBlock(i);
        message.send_async(&mut *stream).await?;
        let message = Message::receive_async(&mut *stream).await?;

        match message {
            Message::NewBlock(block) => {
                let mut blockchain = crate::BLOCKCHAIN.write().await;
                blockchain.add_block(block)?;
            }
            _ => {
                println!("unexpected message from {}", node);
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    Ok(())
}
