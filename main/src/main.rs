use cli::CommandLine;
mod cli; 
use wallet;
use blockchain;

fn main() { 
    let mut cli = CommandLine {
        blockchain : None
    };
    cli.run();

}



