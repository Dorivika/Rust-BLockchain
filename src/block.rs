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
pub struct Block{
    pub hash: Option<Arc<str>>,
    pub transactions: Option<Rc<Vec<Transaction>>>,
    pub prev_hash: Option<Arc<str>>,
    pub nonce: Option<Rc<i32>>,
}

impl Block {
    pub fn hash_transacitons(&self) -> Vec<u8>{
        let mut tx_hashes: Vec<Vec<u8>> = Vec::new();
        let tx_hash: Vec<u8>;
        for tx in self.transactions.as_ref().unwrap().iter() {
            tx_hashes.push(tx.id.as_deref().unwrap().to_vec());
        }
        let mut hasher = Sha256::new();
        let binding = bincode::serialize(&tx_hashes).unwrap();
        let tx_hashes = binding.as_slice();
        hasher.update(&tx_hashes);
        tx_hash = hasher.finalize().to_vec();
        tx_hash
    }
}

pub fn Genesis(coinbase : Transaction) -> Block {
    create_block(Some(Rc::new(vec![coinbase])), Arc::from(""))
}

pub fn create_block<'a>(txs: Option<Rc<Vec<Transaction>>>, prev_hash: Arc<str>) -> Block {
    let mut block = Block {
        hash: None,
        transactions: txs,
        prev_hash : Some(prev_hash),
        nonce: Some(0.into()),
    };
    let pow = proof::new_proof(&block);
    let (nonce, hash) = pow.run().unwrap();
    let hash = Some(hash);
    let nonce_rc = Rc::new(nonce);
    block.nonce = Some(nonce_rc);
    block.hash = Some(Arc::from(hash.unwrap().as_str()));
    block
}

