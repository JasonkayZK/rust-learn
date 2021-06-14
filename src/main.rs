extern crate functional_features;

use std::{env, process};

fn main() {
    let config = functional_features::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("** Searching for {}, in file {} **\n", config.query, config.filename);

    if let Err(e) = functional_features::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
