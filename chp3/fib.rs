use std::env;
use std::process;

mod fib_lib;

use fib_lib::fib_i64;

fn main() {
    let mut args = env::args();
    if args.len() < 2 {
        println!("\nfib: \n    Usage `$ fib 1000`.\n");
        process::exit(0);
    }

    args.next(); // skip first arg (binary file name)
    let nearest_to: i64 = match args.next() {
        Some(x) => x.trim().parse().expect("Expecting an integer"),
        _ => 0
    };

    println!("{:?}", fib_i64(nearest_to));
}
