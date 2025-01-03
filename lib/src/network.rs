use crate::{
    crypto::PublicKey,
    types::{Block, Transaction, TransactionOutput},
};

pub enum Message {
    /// fetch all utxos belonging to a public key
    FetchUTXOs(PublicKey),
    /// utxos belonging to a public key. Bool determines if marked
    UTXOs(Vec<(TransactionOutput, bool)>),
    /// send a transaction to the network
    SubmitTransaction(Transaction),
    /// Broadcast a new transaction to other nodes
    NewTransaction(Transaction),
    /// Ask the node to prepare the optimal block template
    /// with the coinbase transaction paying the specified 
    /// public key
    FetchTemplate(PublicKey),
    /// the template 
    Template(Block),
    /// Ask the node to valide a block template
    /// this is to prevent the node from mining an invalid block
    ValidateTemplate(Block),
    /// if template is valid
    TemplateValidity(bool),
    /// submit a mined block to a node
    SubmitTemplate(Block),
    /// Ask a node to report all the other nodes it knows about 
    DiscoverNodes,
    /// This is the response to DiscoverNodes
    NodeList(Vec<String>),
    /// Ask a node whats the higheest block it knows about in comparison 
    /// to the local blockchain 
    AskDifference(u32),
    /// This is the response to AskDifference 
    Difference(i32),
    /// Ask a node to send a block with the specified height
    FetchBlock(usize),
    /// Broadcast a new block to other nodes
    NewBlock(Block),
}
