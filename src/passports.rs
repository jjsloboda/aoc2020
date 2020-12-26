
use std::collections::HashSet;
use std::array::IntoIter;
use std::iter::FromIterator;

use regex::Regex;

use crate::lib;

pub fn run(part: u32) {
    let lines: Vec<String> = lib::parse::read_to_vec::<String>("input/04.txt")
        .split(|s| s.is_empty())
        .map(|l| l.join(" "))
        .collect();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<key>[[:lower:]]{3}):(?P<value>\S+)").unwrap();
        static ref REQD_KEYS: HashSet<&'static str> = HashSet::from_iter(
            IntoIter::new(["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", /*"cid",*/]));
    }
    let mut num_valid = 0;
    for l in lines {
        println!("line: {}", l);
        let keys: Vec<_> = RE.captures_iter(&l)
            .map(|g| String::from(&g["key"]))
            .collect();
        println!("keys: {:?}", keys);
        if keys.iter()
                .map(|k| k.as_str())
                .collect::<HashSet<_>>()
                .is_superset(&REQD_KEYS) {
            num_valid += 1;
        }
    }
    println!("num valid: {}", num_valid);
}
