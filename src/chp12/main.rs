use std::env;
use std::process;

mod minigrep;

use minigrep::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error occurred: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(cfg) {
        eprintln!("App Error: {}", e);
        process::exit(1);
    }
}
