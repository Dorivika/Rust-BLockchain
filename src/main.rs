mod block;
mod proof;

use block::BlockChain;

fn main() {
    let mut chain = BlockChain::init_blockchain();
    chain.AddBlock(Some("First Block".to_string()));
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



