use std::{ops::Deref, rc::Rc};
use wallet::wallet;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::blockchain::BlockChain;

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TxOutput {
    pub value : Rc<i32>,
    pub pubkeyhash : Option<Rc<[u8]>>, 
}

impl TxOutput {
    pub fn lock(&mut self, address: Vec<u8>) {
        let mut pubkeyhash = bas58::decode(&address);
        pubkeyhash = pubkeyhash[1..(pubkeyhash.len()-4)];
        self.pubkeyhash = Some(Rc::new(pubkeyhash));
    }

    pub fn is_locked_with_key(pubkeyhash : &[u8]) -> bool {
        self.pubkeyhash.deref() == pubkeyhash

    }

    pub fn new_tx_output(value : i32, address : Vec<u8>) -> TxOutput {
        let mut txo = TxOutput{
            value : Rc::new(value),
            pubkeyhash : None,
        };
        txo = txo.lock(address)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TxInput {
    pub id : Vec<u8>,
    pub out : Rc<i32>,
    pub sig : Rc<str>,
    pub pubkey : Rc<[u8]]>
}

impl TxInput {
    pub fn uses_input(&self, pubkeyhash: &[u8]) -> bool {
        let lockinghash = wallet::pub_key_hash(self.pubkey.deref());
        lockinghash == pubkeyhash
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
    let mut tx = Transaction {
        id : None,
        inputs : vec![txin],
        outputs : vec![txout],
    };
    tx = tx.set_id();
    tx

}

pub fn new_transaction<'a >(chain : &'a BlockChain, from :&str, to : &str, amount : &i32) -> Transaction {
    let mut inputs : Vec<TxInput> = vec![];
    let mut outputs : Vec<TxOutput> = vec![];

    let (acc,valid_outs) = chain.find_spos(from, amount);
    if acc < *amount {
        println!(" ERROR NOT ENOUGH FUNDS");
        std::process::exit(0)
    };

    for (txid , outs) in valid_outs {
        let txid = hex::decode(&txid).unwrap();
        for out in outs {
            let input = TxInput {
                id : txid.clone(),
                out : Rc::new(out),
                sig : Rc::from(from),
            };
            inputs.push(input);
        }
    }

    outputs.push(TxOutput {
        value : Rc::new(*amount),
        pubkey : Rc::from(to),
    });

    if acc > *amount {
        outputs.push(TxOutput {
            value : Rc::new(acc-*amount),
            pubkey : Rc::from(from),
        });
        println!("{}",acc-*amount)
    }

    let tx = Transaction {
        id : None,
        inputs,
        outputs,
    };
    let tx = tx.set_id();
    tx
}