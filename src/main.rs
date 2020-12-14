mod lib;
mod expenses;

use std::env;

fn call_solution(day: u32, part: u32) {
    match day {
        1 => expenses::run(part),
        _ => panic!("invalid or not implemented"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 3, "Need to pass day and part to run");
    let (day, part) = (args[1].parse().unwrap(), args[2].parse().unwrap());
    call_solution(day, part);
}
