#![feature(array_value_iter)]

#[macro_use] extern crate lazy_static;

mod lib;

mod expenses;    // day 1
mod passwords;   // day 2
mod slopetrees;  // day 3
mod passports;   // day 4

use std::env;

fn call_solution(day: u32, part: u32) {
    match day {
        1 => expenses::run(part),
        2 => passwords::run(part),
        3 => slopetrees::run(part),
        4 => passports::run(part),
        _ => panic!("invalid or not implemented"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 3, "Need to pass day and part to run");
    let (day, part) = (args[1].parse().unwrap(), args[2].parse().unwrap());
    call_solution(day, part);
}
