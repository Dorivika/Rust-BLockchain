use std::{borrow::BorrowMut, clone, collections::HashMap, fs::File, hash::Hash, io::{Read, Write}, ops::Deref, sync::Mutex};
use serde::Serialize;
use lazy_static::lazy_static;
use serde::Deserialize;
use crate::{block::{self, *}};

lazy_static! {
    pub static ref DATABASE: Mutex<HashMap<String, Vec<u8>>> = Mutex::new(load_database("db.bin").unwrap());
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
            let gen_hash = genesis.hash.clone().unwrap();
            let serialized_genesis = bincode::serialize(&genesis).unwrap();
            DATABASE.lock().unwrap().insert(gen_hash.clone(), serialized_genesis);
            DATABASE.lock().unwrap().insert("lh".to_string(),gen_hash.clone().into_bytes());
            save_database(&DATABASE.lock().unwrap(), "db.bin").unwrap();
            BlockChain {
                lasthash : gen_hash.clone(),
                database : & DATABASE,
            }
            
        } else {
            let lasthash = String::from_utf8_lossy(&DATABASE.lock().unwrap().get("lh").unwrap()).to_string();
            save_database(&DATABASE.lock().unwrap(), "db.bin").unwrap();
            BlockChain {
                lasthash,
                database: & DATABASE,
            }
        }
    }

    pub fn add_block(&mut self, data : String) {
        let binding = self.database.lock().unwrap();
        let item = binding.get("lh").unwrap();
        let item = String::from_utf8_lossy(item).to_string();

        let new_block = create_block(Some(data), Some(item.clone()));
        let serialized_newblock = bincode::serialize(&new_block).unwrap();

        self.database.lock().unwrap().insert(new_block.hash.clone().unwrap(),serialized_newblock);

        self.database.lock().unwrap().insert("lh".to_string(), new_block.hash.unwrap().into_bytes());

        save_database(&self.database.lock().unwrap(), "db.bin").unwrap();

    }   

    pub fn iterator(&self) -> BlockChainIterator {
        BlockChainIterator {
            currhash : self.lasthash.clone(),
            database : self.database,
        }
    } 
}

impl<'a> Iterator for BlockChainIterator<'a> {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        let item = &self.currhash;
        let block_encoded = self.database.lock().unwrap().get(item).unwrap().clone();

        let block: Block = match bincode::deserialize(&block_encoded) {
            Ok(block) => block,
            Err(_) => return None,
        };

        self.currhash = block.prev_hash.clone().unwrap();
        Some(block)
    }
}

pub fn save_database(database: &HashMap<String, Vec<u8>>, filename: &str) -> std::io::Result<()> {
    let serialized_database = bincode::serialize(database).unwrap();
    let mut file = match File::open(format!("{}",filename)) {
        Ok(file) => file,
        Err(e) => {
            println!("error HERE :  {}", e);
            return Err(e)
        },
    };
    file.write_all(&serialized_database)?;
    Ok(())
}

pub fn load_database(filename: &str) -> std::io::Result<HashMap<String, Vec<u8>>> {
    let mut file = match File::open(format!("{}",filename)) {
        Ok(file) => file,
        Err(e) => {
            println!("error HERE :  {}", e);
            return Err(e)
        },
    };
    let mut serialized_database = Vec::new();
    let _ = match file.read_to_end(&mut serialized_database) {
        Ok(file) => file,
        Err(e) => {
            println!("error HERE :  {}", e);
            return Err(e.into()); // Convert the error type using the `Into` trait
        },
    };
    let database = match bincode::deserialize(&serialized_database) {
        Ok(data) => {
            data
        },
        Err(e) => {
            println!("unable to put read data into memory");
            Err(e)
        }
    }
}