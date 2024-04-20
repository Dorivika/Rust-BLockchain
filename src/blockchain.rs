use std::{collections::HashMap, fs::{File, OpenOptions}, io::{Read, Write}, ops::Deref, sync::{Arc, Mutex}};
use lazy_static::lazy_static;
use num::ToPrimitive;
use crate::{block::{self, *}, transaction::{self, Transaction, TxOutput}};
use hex;
lazy_static! {
    pub static ref DATABASE: Mutex<HashMap<String, Vec<u8>>> = Mutex::new(load_database("db.txt").unwrap());
}

#[derive(Debug)]
pub struct BlockChain <'a> {
    lasthash : Arc<str>,
    database : &'a Mutex<HashMap<String, Vec<u8>>>,
}


pub struct BlockChainIterator <'a> {
    currhash : Arc<str>,
    database : &'a Mutex<HashMap<String, Vec<u8>>>,
}


impl <'a>BlockChain <'a>{
    pub fn iterator(&self) -> BlockChainIterator {
        BlockChainIterator {
            currhash : self.lasthash,
            database : self.database,
        }
    } 

    pub fn init_blockchain(address : String) -> BlockChain<'a>{
        if DATABASE.lock().unwrap().is_empty() == true {
            //SET GENESIS DATA
            let cbtx = transaction::coin_base_tx(address, "genesisData".to_string());
            let genesis = block::Genesis(cbtx);
            println!("Genesis proved");
            let serialized_genesis = bincode::serialize(&genesis).unwrap();
            let mut binding = match DATABASE.lock() {
                Ok(db) => db,
                Err(e) => {
                    println!("Unable to acquire mutex lock in init_blockchain");
                    std::process::exit(0)
                }
            };
            let gen_hash = *genesis.hash.unwrap();
            binding.insert(gen_hash.to_string(), serialized_genesis);
            binding.insert("lh".to_string(), gen_hash.as_bytes().to_vec());
            save_database(&binding.deref(), "db.txt").unwrap();
            BlockChain {
                lasthash : Arc::from(gen_hash),
                database : & DATABASE,
            }
            
        } else {
            println!("Already a chain in database");
            std::process::exit(0)
        }
    }
    pub fn continue_blockchain(address : String) -> BlockChain<'a>{
        if DATABASE.lock().unwrap().is_empty() == true {
            println!("Database Doest not exist. Pro Tip : Create One");
            std::process::exit(0);
            
        } else {
            let cbtx = transaction::coin_base_tx(address, "genesisData".to_string());
            let genesis = block::Genesis(cbtx);
            println!("Genesis proved");
            let serialized_genesis = bincode::serialize(&genesis).unwrap();
            let mut binding = match DATABASE.lock() {
                Ok(db) => db,
                Err(e) => {
                    println!("Unable to acquire mutex lock in init_blockchain");
                    std::process::exit(0)
                }
            };
            let gen_hash = *genesis.hash.unwrap();
            binding.insert(gen_hash.to_string(), serialized_genesis);
            binding.insert("lh".to_string(), gen_hash.as_bytes().to_vec());
            save_database(&binding.deref(), "db.txt").unwrap();
            BlockChain {
                lasthash : Arc::from(gen_hash),
                database : & DATABASE,
            }
        }
    }

    pub fn add_block(&mut self, data : Transaction) {
        let mut binding = self.database.lock().unwrap();
        let item = binding.get("lh").unwrap();
        let item = std::str::from_utf8(item).unwrap();

        let new_block = create_block(Some(vec![data]), Arc::new(item));
        println!("BLOCK CREATED");
        println!("{:?}", new_block);
        let serialized_newblock = bincode::serialize(&new_block).unwrap();
        println!("Block Serialized");
        let new_block_hash = *new_block.hash.unwrap();
        binding.insert(new_block_hash.to_string(),serialized_newblock);
        
        binding.insert("lh".to_string(), new_block_hash.as_bytes().to_vec());

        save_database(&binding, "db.txt").unwrap();

    }   

    pub fn find_unspent_tx<'b>(&'b self, address : &str) -> Vec<&'b Transaction> {
        let mut unspent_tx : Vec<&Transaction>;
        let mut spent_txos : HashMap<String, Vec<&i32>>;

        let mut iter = self.iterator();

        loop {
            let block = iter.next().unwrap();
            for tx in block.transactions.unwrap() {
                let tx_as_slice = bincode::serialize(&tx).unwrap().as_slice();
                let txid = hex::encode(tx_as_slice);

                'outputs: for (outidx, out) in tx.outputs.iter().enumerate() {
                    if spent_txos.get(&txid) == None {
                        for spentout in spent_txos.get(&txid).unwrap() {
                            if *spentout == &outidx.to_i32().unwrap(){
                                continue 'outputs;
                            }
                        }
                    }if out.can_be_unlock(address) {
                        unspent_tx.push(&tx);
                    }
                } if tx.is_coinbase() == false {
                    for _in in tx.inputs {
                        if _in.can_unlock(address) {
                            let in_txid = hex::encode(_in.id);
                            spent_txos.entry(in_txid).or_insert(Vec::new()).push(&_in.out);
                        }
                    }
                }

            }
            if block.prev_hash.unwrap().len() == 0 {
                break;
            }
        }
        unspent_tx
    }

    pub fn find_utxos(& self, address : String) -> Vec<TxOutput> {
        let mut utxo :Vec<TxOutput>;
        let usnpenttrans = self.find_unspent_tx(&address.as_str());
        for tx in usnpenttrans {
            for out in tx.outputs {
                if out.can_be_unlock(&address){
                    utxo.push(out)
                }
            }
        };
        return utxo;

    }

}

impl<'a> Iterator for BlockChainIterator<'a> {
    type Item = Block<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let item = &self.currhash;
        let block_encoded = self.database.lock().unwrap().get(&item.to_string()).unwrap().clone();

        let block: Block = match bincode::deserialize(&block_encoded) {
            Ok(block) => block,
            Err(_) => return None,
        };

        self.currhash = (*block.prev_hash.unwrap()).into();
        Some(block)
    }
}

pub fn save_database(database: &HashMap<String, Vec<u8>>, filename: &str) -> std::io::Result<()> {
    let serialized_database = bincode::serialize(database).unwrap();
    let mut file = match OpenOptions::new().write(true).open(format!("{}",filename)) {
        Ok(file) => file,
        Err(e) => {
            println!("error HERE :  {}", e);
            return Err(e)
        },
    };
    match file.write_all(&serialized_database) {
        Ok(_) => {
            println!("saved file successfully")
        },
        Err(e) => {
            println!("ERROR IN SAVING {}", e);
            return Err(e)
            
        }
    };
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
    let _ = match file.read_to_end(&mut serialized_database)  {
        Ok(sd) => sd,
        Err(e) => {
            println!("ERROR HERE : {}", e);
            return Err(e);
        } 
    };
    if serialized_database.len() < 1 {
        Ok(HashMap::new())
    } else {
        let database = bincode::deserialize(&serialized_database).unwrap();
        Ok(database)
    }
    
    
}