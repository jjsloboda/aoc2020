
use std::collections::HashSet;
use std::array::IntoIter;
use std::iter::FromIterator;

use regex::Regex;

use crate::lib;

fn between(num: &str, lower: i64, upper: i64) -> bool {
    if let Ok(n) = num.parse::<i64>() {
        n >= lower && n <= upper
    } else {
        false
    }
}

fn is_valid(passport: &Vec<(String, String)>) -> bool {
    for (k, v) in passport {
        if !match k.as_str() {
            "byr" => between(&v, 1920, 2002),
            "iyr" => between(&v, 2010, 2020),
            "eyr" => between(&v, 2020, 2030),
            "hgt" => {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"(\d+)(cm|in)").unwrap();
                }
                if let Some(caps) = RE.captures(v.as_str()) {
                    let (n, units) = (&caps[1], &caps[2]);
                    if units == "cm" {
                        between(n, 150, 193)
                    } else { // units == "in"
                        between(n, 59, 76)
                    }
                } else {
                    false
                }
            },
            "hcl" => {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
                }
                RE.is_match(&v)
            },
            "ecl" => {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
                }
                RE.is_match(&v)
            },
            "pid" => {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"^\d{9}$").unwrap();
                }
                RE.is_match(&v)
            },
            _ => true,
        } {
            println!("fail: k={}, v={}", k, v);
            return false;
        }
        println!("pass: k={}, v={}", k, v);
    }
    true
}

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
        let passport_info: Vec<(_, _)> = RE.captures_iter(&l)
            .map(|g| (String::from(&g["key"]), String::from(&g["value"])))
            .collect();
        if passport_info.iter()
                .map(|k| k.0.as_str())
                .collect::<HashSet<_>>()
                .is_superset(&REQD_KEYS) {
            if part == 1 || is_valid(&passport_info) {
                num_valid += 1;
            }
        }
    }
    println!("num valid: {}", num_valid);
}
