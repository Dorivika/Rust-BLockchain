use std::env;

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
            //exit program
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
            println!("prev hash {}", block.data.clone().unwrap());
            println!("prev hash {}", block.hash.clone().unwrap());
            
            let pow = proof::new_proof(&block);
            println!("POW : {}", proof::validate(&pow));

            if block.prev_hash.clone().unwrap().len() == 0{
                break;
            }

        }
    }

    fn run(&mut self) {
        let args: Vec<String> = env::args().collect();

        let mut opts = Options::new();
        opts.optflag("", "add", "add a block to the chain");
        opts.optflag("", "print", "prints the blocks");

        let matches = match opts.parse(&args[1..]) {
            Ok(m) => { m }
            Err(f) => { panic!("{}", f.to_string()) }
        };

        if matches.opt_present("add") {
            let data = matches.free.join(" ");
            self.add_block(data);
        } else if matches.opt_present("print") {
            self.print_chain();
        } else {
            self.print_usage();
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



