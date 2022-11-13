use std::{env, process};

use minigrep::mini_grep::*;

fn main() {
    let config = Config::new(env::args());

    if config.is_err() {
        println!("Problem parsing arguments: {}", config.err().unwrap());
        process::exit(1);
    }

    if let Err(e) = run(config.unwrap()) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
