use crate::proof;

use std::fmt;
use bincode::{self};
use serde::ser::SerializeStruct;
use serde::ser::Serializer;
use serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};

#[derive(Debug)]
pub struct Block {
    pub hash: Option<String>,
    pub data: Option<String>,
    pub prev_hash: Option<String>,
    pub nonce: Option<i32>,
}
struct BlockVisitor;

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


impl<'de> Visitor<'de> for BlockVisitor {
    type Value = Block;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct Block")
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<Block, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let hash = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
        let prev_hash = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(1, &self))?;
        let data = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(2, &self))?;
        let nonce = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(3, &self))?;
        Ok(Block {
            hash,
            prev_hash,
            data,
            nonce,
        })
    }
}

impl<'de> serde::Deserialize<'de> for Block {
    fn deserialize<D>(&self,deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct("Block", &["hash", "prev_hash", "data", "nonce"], BlockVisitor)
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