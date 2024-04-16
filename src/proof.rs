const DIFFICULTY:u32 = 10;


use crate::block::Block;
use sha2::{Digest, Sha256};

use num::{bigint::BigInt, Num};

pub struct Pow<'a> {
    Block: &'a Block,
    Target: BigInt,
}

pub fn new_proof<'a>(b: &'a Block) -> Pow<'a> {
    let target: BigInt = BigInt::from(1);
    let target = target << (256 - DIFFICULTY);
    Pow {
        Block : b,
        Target : target,
    }
}

impl  <'a>Pow <'a> {
    pub fn run(&self) -> (i32, String) {
    
        let mut nonce = 0;
    
        while nonce < i32::MAX {
            let data = Pow::init_data(self, nonce);
    
            let mut hasher = Sha256::new();
            hasher.update(data.as_bytes());
            let hash = hasher.finalize();
            let hash_as_str = format!("{:x}",hash);
            println!("hash : {} ", hash_as_str);
    
            let hash_bigint = BigInt::from_str_radix(&hash_as_str, 16).unwrap();
    
            if hash_bigint < self.Target {
                return (nonce, hash_as_str);
            }
    
            nonce+=1
        }
        (0, "".to_string())
    }
    fn init_data (p : &'a Pow , nonce : i32) -> String {
        let data = format!("{},{},{},{:?},{:?}", 
        p.Block.prev_hash.as_deref().unwrap_or(""),
        p.Block.data.as_deref().unwrap_or(""),
        "".to_string(),
        nonce.to_ne_bytes(),
        DIFFICULTY.to_ne_bytes()
        );
        data
    }
    
}



pub fn validate(p : &Pow) -> bool {
    let data = Pow::init_data(p, p.Block.nonce.unwrap());

    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash = hasher.finalize();

    let hash_as_str = format!("{:x}", hash);
    let hash = BigInt::from_str_radix(&hash_as_str, 16).unwrap();

    if hash<p.Target {
        return true
    };
    false
}
