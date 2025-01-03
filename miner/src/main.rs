use std::{env, process::exit, usize};

use btc_lib::{types::Block, util::Saveable};

fn main() {
    // parse block path and the steps count from the first and
    // second argument
    let (path, steps) = if let (Some(arg), Some(arg2)) = (env::args().nth(1), env::args().nth(2)) {
        (arg, arg2)
    } else {
        eprintln!("usage: miner <block_file> <steps>");
        exit(1);
    };

    let steps = if let Ok(s @ 1..=usize::MAX) = steps.parse() {
        s
    } else {
        eprintln!("<steps> should be a positive integer");
        exit(1);
    };

    // load block from file
    let og_block = Block::load_from_file(path).expect("failed to load block");
    let mut block = og_block.clone();

    while !block.header.mine(steps) {
        println!("mining...");
    }

    println!("original: {:#?}", og_block);
    println!("hash: {}", og_block.header.hash());
    //mined block
    println!("final: {:#?}", block);
    println!("hash: {}", block.header.hash());
}
