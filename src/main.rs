use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct Block {
    hash: Option<String>,
    data: Option<String>,
    prev_hash: Option<String>,
}

#[derive(Debug)]
struct BlockChain {
    blocks : Vec<Block>,
}

impl BlockChain {
    pub fn AddBlock(& mut self, data : Option<String>){
       let  prev_block = &self.blocks[self.blocks.len() -1];
       let new_block = Block::create_block(data, prev_block.hash.clone());
       self.blocks.push(new_block)
    }

    fn Genesis() -> Block {
        Block::create_block(Some("Genesis".to_string()), Some("".to_string()))
    }

    pub fn init_blockchain() -> BlockChain {
        let mut new_blockchain = vec![];
        new_blockchain.push(BlockChain::Genesis());
        BlockChain {
            blocks : new_blockchain,
        }
    }
}


impl Block {
    pub fn create_block(data : Option<String>, prev_hash: Option<String>) -> Block {
        let mut block = Block {
            hash : None,
            data,
            prev_hash
        };

        block.derive_hash();
        block
    }

    fn derive_hash(&mut self) {
        let mut hasher = Sha256::new();

        let serialized_data = format!(
            "{},{},{}",
            self.data.as_deref().unwrap_or(""),
            self.hash.as_deref().unwrap_or(""),
            self.prev_hash.as_deref().unwrap_or("")
        );

        hasher.update(serialized_data.as_bytes());

        let hash_result = hasher.finalize();
        self.hash = Some(format!("{:x}", hash_result));
    }
}

fn main() {
    let mut chain = BlockChain::init_blockchain();
    chain.AddBlock(Some("First Block".to_string()));
    chain.AddBlock(Some("second Block".to_string()));
    chain.AddBlock(Some("Thrid Block".to_string()));

    for blocks in chain.blocks {
        println!("{}", blocks.prev_hash.unwrap());
        println!("{}", blocks.data.unwrap());
        println!("{}", blocks.hash.unwrap());

    }
}



