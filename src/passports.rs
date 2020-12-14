
use std::collections::HashSet;

use regex::Regex;

use crate::lib;

pub fn run(part: u32) {
    let lines: Vec<String> = lib::parse::read_to_vec::<String>("input/04.txt")
        .split(|s| s.is_empty())
        .map(|l| l.join(" "))
        .collect();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?:(?P<key>[[:lower:]]{3}):(?P<value>.+) ?)+").unwrap();
        static ref required_keys: HashSet<&'static str> = HashSet::from_iter(
            IntoIter::new(["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"]));
    }
    let mut num_valid = 0;
    for l in lines {
        let keys = RE.captures_iter(l).map(|g| &g["key"]).collect();
    }
    println!("num valid: {}", num_valid);
}
