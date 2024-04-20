use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;
use crate::proof;
use crate::transaction::Transaction;
use serde::Deserialize;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;

#[derive(Debug,Serialize, Deserialize)]
pub struct Block<'a> {
    pub hash: Option<Arc<&'a str>>,
    pub transactions: Option<Vec<Transaction>>,
    pub prev_hash: Option<Arc<&'a str>>,
    pub nonce: Option<Rc<i32>>,
}

impl <'a>Block<'a> {
    pub fn hash_transacitons(&self) -> Vec<u8>{
        let mut tx_hashes: Vec<Vec<u8>> = Vec::new();
        let tx_hash: Vec<u8>;
        for tx in &self.transactions.unwrap() {
            tx_hashes.push(tx.id.unwrap().clone());
        }
        let mut hasher = Sha256::new();
        let tx_hashes = bincode::serialize(&tx_hashes).unwrap().as_slice();
        hasher.update(&tx_hashes);
        tx_hash = hasher.finalize().to_vec();
        tx_hash
    }
}

pub fn Genesis(coinbase : Transaction) -> Block<'static> {
    create_block(Some(vec![coinbase]), Arc::new("".deref()))
}

pub fn create_block<'a>(txs: Option<Vec<Transaction>>, prev_hash: Arc<&str>) -> Block<'a> {
    let mut block = Block {
        hash: None,
        transactions: txs,
        prev_hash : Some(prev_hash),
        nonce: Some(0.into()),
    };
    let pow = proof::new_proof(&block);
    let (nonce, hash) = pow.run().unwrap();
    let hash = Some(hash);
    block.nonce = Some(Rc::new(nonce));
    block.hash = Some(Arc::new(hash.unwrap()));
    block
}

