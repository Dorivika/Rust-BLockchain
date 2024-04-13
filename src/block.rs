use core::hash;

use sha2::{Digest, Sha256};

use crate::proof;


#[derive(Debug)]
pub struct Block {
    pub hash: Option<String>,
    pub data: Option<String>,
    pub prev_hash: Option<String>,
    pub nonce : Option<i32>,
}

#[derive(Debug)]
pub struct BlockChain {
    pub blocks : Vec<Block>,
}

impl BlockChain {
    pub fn AddBlock(& mut self, data : Option<String>){
       let  prev_block = &self.blocks[self.blocks.len() -1];
       let new_block = create_block(data, prev_block.hash.clone());
       self.blocks.push(new_block)
    }

    fn Genesis() -> Block {
        create_block(Some("Genesis".to_string()), Some("".to_string()))
    }

    pub fn init_blockchain() -> BlockChain {
        let mut new_blockchain = vec![];
        new_blockchain.push(BlockChain::Genesis());
        BlockChain {
            blocks : new_blockchain,
        }
    }
}


    fn create_block(data: Option<String>, prev_hash: Option<String>) -> Block {

        //fix this bullshit

        let data_m = data.clone();
        let prev_hash_m = prev_hash.clone();
        let mut block = Block {
            hash: None,
            data,
            prev_hash,
            nonce: Some(0),
        };

        let pow = proof::new_proof(&block);
        let (nonce, hash) = pow.run();
        let nonce = Some(nonce);
        let hash = Some(hash);
        let mut block = Block {
            hash,
            data: data_m,
            prev_hash : prev_hash_m,
            nonce,
        };
        block
    }