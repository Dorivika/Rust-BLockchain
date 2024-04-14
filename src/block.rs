use crate::proof;

use bincode::{self};
use serde::ser::SerializeStruct;
use serde::ser::Serializer;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Block {
    pub hash: Option<String>,
    pub data: Option<String>,
    pub prev_hash: Option<String>,
    pub nonce: Option<i32>,
}

impl serde::Serialize for Block {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Block", 3)?;
        s.serialize_field("field1", &self.hash)?;
        s.serialize_field("field2", &self.prev_hash)?;
        s.serialize_field("field3", &self.data)?;
        s.serialize_field("field4", &self.nonce)?;
        // add more fields as needed
        s.end()
    }
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