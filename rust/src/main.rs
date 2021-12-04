use std::env;
use std::process;

mod lib;

use lib::Config;
use lib::run;

fn main() {
    let args = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|_err| {
        process::exit(1);
    });

    if let Err(_e) = run(config) {
        process::exit(1);
    };
}
