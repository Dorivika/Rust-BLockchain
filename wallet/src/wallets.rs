use std::{collections::HashMap, error::Error, fs::{File, OpenOptions}, io::{Read, Write}, ops::Deref};
use ring::error;

use crate::wallet::Wallet;
struct  Wallets {
    wallets : Option<HashMap<String, Wallet>>
}

impl Wallets {
    fn save_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let ser_wallet = bincode::serialize(&self.wallets)
                                .expect("failed to serialze wallets ");
        let mut file = OpenOptions::new().write(true).open(format!("saved_wallets"))
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        file.write_all(&ser_wallet)
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(())
    }

    fn load_file(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::open(format!("saved_wallets"))
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        let mut ser_wallets : Vec<u8> = Vec::new();

        file.read_to_end(&mut ser_wallets)
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        
        if ser_wallets.len() < 1{
            self.wallets = Some(HashMap::new())
        }
        self.wallets = bincode::deserialize(&ser_wallets).expect("unable to derserialize wallets");
        Ok(())
    }
    
    fn create_wallets() -> Result<(), Box<dyn std::error::Error>> {
        let mut wallets = Wallets {
            wallets : None,
        };
        let err = wallets.load_file();
        println!("{:?}", err);

        Ok(())
    }

    fn get_wallet(&self, address: String) -> Wallet {
        let ws = self.wallets.as_ref().unwrap().get(&address).unwrap().clone();
        ws
    }
}