use std::env;
use std::process;

mod lib;

use lib::Config;
use lib::run;

fn main() {
    let args = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("couldn't parse the arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    };
}
