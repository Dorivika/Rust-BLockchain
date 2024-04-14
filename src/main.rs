use std::collections::HashMap;

use blockchain::BlockChain;

mod block;
mod proof;
mod blockchain;


struct CommandLine <'b> {
    blockchain : &'b BlockChain<'b>,
}

impl <'b>CommandLine<'b> {
    fn print_usage(&self) {}
} 
fn main() {
     
    let mut chain = blockchain::BlockChain::init_blockchain();
    BlockChain::<'_>::AddBlock(Some("First Block".to_string()));
    chain.AddBlock(Some("second Block".to_string()));
    chain.AddBlock(Some("Thrid Block".to_string()));

    for blocks in &chain.blocks {
        println!(" Prev hash : {}", blocks.prev_hash.as_ref().unwrap());
        println!("Data : {}", blocks.data.as_ref().unwrap());
        println!("Hash : {}", blocks.hash.as_ref().unwrap());
        let pow = proof::new_proof(&blocks);
        
        println!("POW : {}", proof::validate(&pow));

    }
    
}



