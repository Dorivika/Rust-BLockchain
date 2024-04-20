use std::rc::Rc;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub id : Option<Vec<u8>>,
    pub inputs : Vec<TxInput>,
    pub outputs : Vec<TxOutput>,
}

impl Transaction {
    pub fn set_id (mut self) -> Transaction{
        let mut hasher = Sha256::new();
        let tx_as_bytes = match bincode::serialize(&self) {
            Ok(tx) => tx,
            Err(e) => {
                println!("Can't serialize tx_as_bytes");
                std::process::exit(0);
            },
        };
        hasher.update(tx_as_bytes);
        let tx_hash = hasher.finalize();
        self.id = Some(tx_hash.to_vec());
        self
    }
    pub fn is_coinbase(&self) -> bool {
        self.inputs.len() ==1 && self.inputs[0].id.len() == 0 && *self.inputs[0].out == -1
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TxOutput {
    pub value : Rc<i32>,
    pub pubkey : Rc<str>, 
}

impl TxOutput {
    pub fn can_be_unlock(&self, data: &str) -> bool {
        *self.pubkey == *data
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TxInput {
    pub id : Vec<u8>,
    pub out : Rc<i32>,
    pub sig : Rc<str>,
}

impl TxInput {
    pub fn can_unlock(&self, data: &str) -> bool {
        *self.sig == *data
    }
}
pub fn coin_base_tx(to : String, mut data: String) -> Transaction {
    if data == "" {
        data = format!{"Coin to {}", to};
    };

    let txin = TxInput{
        id : Vec::new(),
        out : Rc::new(-1),
        sig : Rc::from(data),
    };

    let txout = TxOutput {
        value : Rc::new(100),
        pubkey : Rc::from(to),
    };
    let tx = Transaction {
        id : None,
        inputs : vec![txin],
        outputs : vec![txout],
    };
    let tx = tx.set_id();
    tx

}