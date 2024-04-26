use cli::CommandLine;

mod cli; 
mod wallet;
use blockchain;

fn main() { 
    let mut cli = CommandLine {
        blockchain : None
    };
    cli.run();

}



