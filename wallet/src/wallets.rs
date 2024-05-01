use std::{collections::HashMap, error::Error, fs::{File, OpenOptions}, io::{Read, Write}, ops::Deref, vec};
use ring::error;

use crate::wallet::{self, Wallet};
pub struct  Wallets {
    wallets : Option<HashMap<String, Wallet>>
}

impl Wallets {
    pub fn save_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let ser_wallet = bincode::serialize(&self.wallets)
                                .expect("failed to serialze wallets ");
        let mut file = OpenOptions::new().write(true).open(format!("savedwallets.txt"))
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        file.write_all(&ser_wallet)
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(())
    }

    pub fn load_file(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::open(format!("savedwallets.txt"))
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        let mut ser_wallets : Vec<u8> = Vec::new();

        file.read_to_end(&mut ser_wallets)
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        
        if ser_wallets.len() < 1{
            self.wallets = Some(HashMap::new());
        }else {
            self.wallets = bincode::deserialize(&ser_wallets).expect("unable to derserialize wallets");
        }
       
        Ok(())
    }
    
    pub fn create_wallets() -> Result<Wallets, Box<dyn std::error::Error>> {
        let mut wallets = Wallets {
            wallets : None,
        };
        let err = wallets.load_file();
        println!("{:?}", err);

        Ok(wallets)
    }

    pub fn get_wallet(&self, address: String) -> Wallet {
        let ws = self.wallets.as_ref().unwrap().get(&address).unwrap().clone();
        ws
    }
    
    pub fn get_all_addresses<'a>(&'a self) -> Vec<&'a str> {
        self.wallets
        .as_ref()
        .unwrap()
        .keys()
        .map(|x| x.as_str()).collect()
    }

    pub fn add_wallet(&mut self) -> String {
        let wallet = wallet::Wallet::make_wallet();
        let address = wallet.address();
        if self.wallets.is_none() {
            self.wallets = Some(HashMap::new());
        }
        self.wallets.as_mut().unwrap().insert(address.clone(), wallet);
        return address;
    }
}