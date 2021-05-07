use std::error::Error;
use std::{fs, env};
use std::env::args;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough args")
        }

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("didnt get a query string")
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("didnt get filename string")
        };

        Ok(Config {query, filename})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;

    println!("with text:\n{}", contents);

    Ok(())
}
