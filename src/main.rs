use std::{env, os::windows::process};

use blockchain::BlockChain;
use getopts::Options;

mod block;
mod proof;
mod blockchain;


struct CommandLine <'b> {
    blockchain : &'b mut BlockChain<'b>,
}

impl <'b>CommandLine<'b> {
    fn print_usage(&self) {
        println!("Usage :");
        println!("add -block BLOCK_DATA - add a block to the chain");
        println!("print - Prints the blocks");
    }

    fn validate_args(&self) {
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            self.print_usage();
            std::process::exit(0);
        }
    }

    fn add_block(& mut self, data: String) {
        self.blockchain.add_block(data);
        println!("Block Added!");
    }

    fn print_chain(&self) {
        let mut iter = self.blockchain.iterator();

        loop {
            let block = iter.next().unwrap();

            println!("prev hash {}", block.prev_hash.clone().unwrap());
            println!("DATA {}", block.data.clone().unwrap());
            println!("block hash {}", block.hash.clone().unwrap());
            
            let pow = proof::new_proof(&block);
            println!("POW : {}", proof::validate(&pow));

            if block.prev_hash.clone().unwrap().len() == 0{
                break;
            }

        }
    }

    fn run(&mut self) {
        self.validate_args();

        let args: Vec<String> = env::args().collect();
        let query = &args[1];
        if query == "add" {
            let data = &args[2];
            self.add_block(data.to_string().clone());
            std::process::exit(0);
        } else if query=="print" {
            self.print_chain();
            std::process::exit(0);
        }
    }

} 
fn main() { 
    let mut chain = blockchain::BlockChain::init_blockchain();
    let mut cli = CommandLine {
        blockchain : & mut chain,
    };
    cli.run()
    
}



