use std::env;
use std::process;

mod fib_lib;

use fib_lib::nth_fib;

fn main() {
    let mut args = env::args();
    if args.len() < 2 {
        println!("\nnth-fib: \n    Usage `$ nth-fib 1000`.\n");
        process::exit(0);
    }
    args.next(); // skip first arg (binary file name)
    let nth: i32 = match args.next() {
        Some(x) => x.trim().parse().expect("Expecting an integer"),
        _ => 0
    };
    println!("{:?}th fib is {:?}", nth, nth_fib(nth));
}
