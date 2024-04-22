use std::env;

use blockchain::BlockChain;

mod block;
mod proof;
mod blockchain;
mod transaction;


struct CommandLine <'b> {
    blockchain : Option<&'b mut BlockChain<'b>>,
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
        let mut iter = chain.iterator();

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

    fn run(&mut self) {
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
            _ => {
                println!("Invalid command");
            }
        }
    
        std::process::exit(0);
    }

} 
fn main() { 
    let mut cli = CommandLine {
        blockchain : None
    };
    cli.run()
}



