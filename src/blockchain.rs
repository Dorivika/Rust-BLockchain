use std::{clone, collections::HashMap, hash::Hash, ops::Deref, sync::Mutex};
use lazy_static::lazy_static;
use serde::Serialize;
use crate::block::{self, *};
lazy_static! {
    static ref DATABASE : Mutex<HashMap<String, Vec<u8>>>  = Mutex::new(HashMap::new());
}

#[derive(Debug)]
pub struct BlockChain <'a> {
    lasthash : String,
    database : &'a Mutex<HashMap<String, Vec<u8>>>,
}


pub struct BlockChainIterator <'a> {
    currhash : String,
    database : &'a Mutex<HashMap<String, Vec<u8>>>,
}


impl <'a>BlockChain <'a>{
    pub fn init_blockchain() -> BlockChain<'a>{
        if DATABASE.lock().unwrap().is_empty() == true {
            let genesis = block::Genesis();
            println!("Genesis proved");
            let gen_hash = genesis.hash.unwrap().clone();
            let serialized_genesis = bincode::serialize(&genesis).unwrap();
            DATABASE.lock().unwrap().insert(gen_hash, serialized_genesis);
            DATABASE.lock().unwrap().insert("lh".to_string(),gen_hash.into_bytes());
            BlockChain {
                lasthash : gen_hash,
                database : &DATABASE,
            }
        } else {
            let lasthash = String::from_utf8_lossy(&DATABASE.lock().unwrap().get("lh").unwrap()).to_string();

            BlockChain {
                lasthash,
                database: &DATABASE,
            }
        }
    }

    pub fn AddBlock(data : String) {
        let item = DATABASE.lock().unwrap().get("lh").unwrap();
        let item = String::from_utf8_lossy(item).to_string();

        let new_block = create_block(Some(data), Some(item.clone()));
        let serialized_newblock = bincode::serialize(&new_block).unwrap();

        DATABASE.lock().unwrap().insert(new_block.hash.unwrap(),serialized_newblock);

        DATABASE.lock().unwrap().insert("lh".to_string(), new_block.hash.unwrap().into_bytes());

        BlockChain {
            lasthash : new_block.hash.unwrap(),
            database : &DATABASE,
        };
    }   

    pub fn iterator(chain : BlockChain) -> BlockChainIterator {
        BlockChainIterator {
            currhash : chain.lasthash,
            database : chain.database,
        }
    }

    pub fn next(iter : &'a mut BlockChainIterator) -> &'a Block {
        let item = iter.currhash;
        let block = DATABASE.lock().unwrap().get(&item).unwrap().clone();

        let block: Block = bincode::deserialize(&block).unwrap();

        iter.currhash = block.prev_hash.unwrap();
        &block
    }

     
}