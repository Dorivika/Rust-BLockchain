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
        self.inputs.len() ==1 && self.inputs[0].id.len() == 0 && self.inputs[0].out == -1
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TxOutput {
    pub value : i32,
    pub pubkey : String, 
}

impl TxOutput {
    pub fn can_be_unlock(&self, data: &String) -> bool {
        self.pubkey == data.clone()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TxInput {
    pub id : Vec<u8>,
    pub out : i32,
    pub sig : String,
}

impl TxInput {
    pub fn can_unlock(&self, data: &String) -> bool {
        self.sig == data.clone()
    }
}
pub fn coin_base_tx(to : String, mut data: String) -> Transaction {
    if data == "" {
        data = format!{"Coin to {}", to};
    };

    let txin = TxInput{
        id : Vec::new(),
        out : -1,
        sig : data.clone(),
    };

    let txout = TxOutput {
        value : 100,
        pubkey : to,
    };
    let tx = Transaction {
        id : None,
        inputs : vec![txin],
        outputs : vec![txout],
    };
    let tx = tx.set_id();
    tx

}