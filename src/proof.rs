const DIFFICULTY:u32 = 13;


use std::rc::Rc;

use crate::block::Block;
use sha2::{Digest, Sha256};

use num::{bigint::BigInt, Num};

pub struct Pow<'a> {
    Block: &'a Block<'a>,
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
    pub fn run(&self) -> Option<(i32, &str)> {
    
        let mut nonce = 0;
    
        while nonce < i32::MAX {
            let data = Pow::init_data(self, Rc::new(nonce));
    
            let mut hasher = Sha256::new();
            hasher.update(data.as_bytes());
            let hash = hasher.finalize();
            let hash_as_str = format!("{:x}",hash).as_str();
            println!("hash : {} ", hash_as_str);
    
            let hash_bigint = BigInt::from_str_radix(&hash_as_str, 16).unwrap();
    
            if hash_bigint < self.Target {
                return Some((nonce, hash_as_str));
            }
            nonce+=1
        }
        None
    }
    fn init_data (p : &'a Pow , nonce : Rc<i32>) -> String {
        let data = format!("{},{:?},{},{:?},{:?}", 
        p.Block.prev_hash.unwrap_or(Rc::new("")),
        p.Block.hash_transacitons(),
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
