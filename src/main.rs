use grep::config::Config;
use std::{env, process};

mod grep;

fn main() {
    // Read arguments from command line
    let args = env::args().collect::<Vec<_>>();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("ERROR::{}", err.msg());
        process::exit(1);
    });

    // Process arguments
    if let Err(e) = grep::run(config) {
        println!("ERROR::{}", e);
        process::exit(2);
    };
}
