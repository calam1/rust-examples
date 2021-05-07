use std::{env, fs, process};
use std::error::Error;
use minigrep::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("problem parsing args {}", err);
        process::exit(1)
    });

    println!("searching for {}", config.query);
    println!("in file {}", config.filename);

    if let Err(e) = run(config) {
        process::exit(1)
    }
}


