use blockchain::BlockChain;
use cli::CommandLine;

use crate::wallet::pub_key_hash;

mod block;
mod proof;
mod blockchain;
mod transaction;
mod cli; 
mod wallet;
fn main() { 
    let mut cli = CommandLine {
        blockchain : None
    };
    cli.run();

}



