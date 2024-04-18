use crate::proof;
use crate::transaction::Transaction;
use serde::Deserialize;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub hash: Option<String>,
    pub transactions: Option<Vec<Transaction>>,
    pub prev_hash: Option<String>,
    pub nonce: Option<i32>,
}

impl Block {
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

pub fn Genesis(coinbase : Transaction) -> Block {
    create_block(Some(vec![coinbase]), Some("".to_string()))
}

pub fn create_block(txs: Option<Vec<Transaction>>, prev_hash: Option<String>) -> Block {
    let mut block = Block {
        hash: None,
        transactions: txs,
        prev_hash,
        nonce: Some(0),
    };
    let pow = proof::new_proof(&block);
    let (nonce, hash) = pow.run();
    let nonce = Some(nonce);
    let hash = Some(hash);
    block.nonce = nonce;
    block.hash = hash;
    block
}

