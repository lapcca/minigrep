use  std::env;
use std::process;

use minigrep::{Config, run};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for <{}> in {}", config.query, config.file_path);

    if let Err(e) = run(config){
        println!("Read error: {e}");
        process::exit(1);
    }

}

