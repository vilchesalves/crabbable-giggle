use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("too few arguments!");
        }

        let query = args[1].to_owned();
        let filename = args[2].to_owned();
        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("search for {} in {}.\n\n", config.query, config.filename);

    let contents = fs::read_to_string(config.filename)?;

    println!("{}", contents);

    Ok(())
}
