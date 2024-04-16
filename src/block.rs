use crate::proof;

use std::fmt;
use bincode::serialize;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub hash: Option<String>,
    pub data: Option<String>,
    pub prev_hash: Option<String>,
    pub nonce: Option<i32>,
}

pub fn Genesis() -> Block {
    
    create_block(Some("Genesis".to_string()), Some("".to_string()))
}

pub fn create_block(data: Option<String>, prev_hash: Option<String>) -> Block {
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