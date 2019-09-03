use std::env;
use std::process;

mod fib_lib;

use fib_lib::nth_fib;

fn main() {
    let mut args = env::args();
    if args.len() < 2 {
        println!("\nnth-fib: \n    Usage `$ fib-nth 1000`.\n");
        process::exit(0);
    }

    args.next(); // skip first arg (binary file name)
    let nth: i32 = match args.next() {
        Some(x) => x.trim().parse().expect("Expecting an integer"),
        _ => 0
    };

    let mut num_suffix = "th";
    if nth > 20 {
        num_suffix = match nth % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th"
        };
    }

    println!("{:?}{:} fib is {:?}", nth, num_suffix, nth_fib(nth));
}
