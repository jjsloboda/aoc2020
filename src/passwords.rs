
use std::str::FromStr;
use regex::Regex;

use crate::lib;

struct Password {
    min: u32,
    max: u32,
    ch: char,
    password: String,
}

impl Password {
    fn valid(self: &Self) -> bool {
        let mut count: u32 = self.password.chars()
            .map(|ch| if ch == self.ch { 1 } else { 0 })
            .sum();
        count >= self.min && count <= self.max
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
            min: captures[1].parse().unwrap(),
            max: captures[2].parse().unwrap(),
            ch: captures[3].parse().unwrap(),
            password: captures[4].parse().unwrap(),
        })
    }
}

pub fn run(part: u32) {
    let passwds = lib::parse::read_to_vec::<Password>("input/02.txt");
    let num_valid: u32 = passwds.iter()
        .map(|p| if p.valid() { 1 } else { 0 })
        .sum();
    println!("num valid passwords: {}", num_valid);
}
