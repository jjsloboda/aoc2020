
use std::str::FromStr;
use regex::Regex;

use crate::lib;

struct Password {
    first: u32,
    second: u32,
    ch: char,
    password: String,
}

impl Password {
    fn valid_count(self: &Self) -> bool {
        let count: u32 = self.password.chars()
            .map(|ch| if ch == self.ch { 1 } else { 0 })
            .sum();
        count >= self.first && count <= self.second
    }

    fn valid_index(self: &Self) -> bool {
        let first_match = self.password.chars().nth(self.first as usize- 1).unwrap() == self.ch;
        let second_match = self.password.chars().nth(self.second as usize- 1).unwrap() == self.ch;
        lib::ops::xor(first_match, second_match)
    }
}

impl FromStr for Password {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+)-(\d+) (.): ([[:lower:]]+)").unwrap();
        }
        let captures = RE.captures(s).unwrap();
        Ok(Password {
            first: captures[1].parse().unwrap(),
            second: captures[2].parse().unwrap(),
            ch: captures[3].parse().unwrap(),
            password: captures[4].parse().unwrap(),
        })
    }
}

pub fn run(part: u32) {
    let passwds = lib::parse::read_to_vec::<Password>("input/02.txt");
    let func = if part == 1 { Password::valid_count } else { Password::valid_index };
    let num_valid: u32 = passwds.iter()
        .map(|p| if func(&p) { 1 } else { 0 })
        .sum();
    println!("num valid passwords: {}", num_valid);
}
