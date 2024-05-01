use std::env;
use blockchain::blockchain::BlockChain;
use blockchain::proof;
use blockchain::transaction;
use wallet::wallet;
use crate::wallet::wallets;

pub struct CommandLine <'b> {
    pub blockchain : Option<&'b mut BlockChain<'b>>,
}

impl <'b>CommandLine<'b> {
    fn print_usage(&self) {
        println!("Usage :");
        println!("getbalance -address ADDRESS -> gets the balance for the given address");
        println!("createblockchain -address ADDRESS -> creates a blockchain");
        println!("printchain -> prints the entire blocchain");
        println!("send -from FROM -to TO -amount AMOUNT -> Send amount from one account to another");
    }

    fn validate_args(&self) {
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            self.print_usage();
            std::process::exit(0);
        }
    }

    fn print_chain(&self) {
        let chain = BlockChain::continue_blockchain("");
        let iter = chain.iterator();

        loop {
            let block = iter.borrow_mut().next().unwrap();

            println!("prev hash {}", block.prev_hash.clone().unwrap());
            println!("block hash {}", block.hash.clone().unwrap());
            
            let pow = proof::new_proof(&block);
            println!("POW : {}", proof::validate(&pow));

            if block.prev_hash.clone().unwrap().len() == 0{
                break;
            }

        }
    }

    fn create_blockchain(&self, address : String) {
        let chain = BlockChain::<'b>::init_blockchain(address);
        println!("Finished Operation")
    }

    fn getbalance(&self, address : String) {
        let chain = BlockChain::continue_blockchain(&address.as_str());

        let mut balance = 0;
        let utxo_s = chain.find_utxos(&address);

        for out in utxo_s {
            balance += *out.value;
        }

        println!("Balance of {} {}",address,balance);
    }

    fn send (&self, from : String, to: String, amount : i32) {
        let mut chain = BlockChain::continue_blockchain(&from);

        let tx = transaction::new_transaction(&chain, &from, &to, &amount);

        chain.add_block(vec![tx]);
        println!("SUCCESS!!")
    }

    pub fn run(&mut self) {
        self.validate_args();
    
        let args: Vec<String> = env::args().collect();
        let command = &args[1];
    
        match command.as_str() {
            "print" => {
                self.print_chain();
            }
            "create" => {
                let address = &args[2];
                self.create_blockchain(address.to_string());
            }
            "balance" => {
                let address = &args[2];
                self.getbalance(address.to_string());
            }
            "send" => {
                let from = &args[2];
                let to = &args[3];
                let amount = args[4].parse::<i32>().expect("Amount should be a number");
                self.send(from.to_string(), to.to_string(), amount);
            }
            "createwallet" => {
                self.create_wallet()
            }
            "listaddresses" => {
                self.list_addresses()
            }
            _ => {
                println!("Invalid command");
            }
      
        }
    
        std::process::exit(0);
    }

    fn list_addresses(&self){
        let wallets = match wallets::Wallets::create_wallets() {
            Ok(wallets) => wallets,
            Err(e) => {
                println!("couldn't create wallets in list_addresses CLI");
                return;
            }
        };
        let addresses = wallets.get_all_addresses();
        for address in addresses {
            println!("{}",address.to_string())
        }
    }

    fn create_wallet(&self) {
        let mut wallets = match wallets::Wallets::create_wallets() {
            Ok(wallets) => wallets,
            Err(e) => {
                println!("couldn't create wallets in list_addresses CLI");
                return;
            }
        };

        let address = wallets.add_wallet();
        let _ = match wallets.save_file() {
            Ok(()) => (),
            Err(e) => {
                println!("Couldn't save wallet, Error : {}", e);
                return;
            }
        };
        println!("New wallet added, address is {}", address);
    }

}