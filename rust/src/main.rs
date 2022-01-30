use std::env;
use std::process;

mod minigrep;
mod cacher;
mod doors;
mod iterators;

mod fifteen;

fn main() {
    let args = env::args();
    let config = minigrep::Config::new(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}
