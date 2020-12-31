
use std::collections::{HashSet, HashMap, VecDeque};
use std::str::FromStr;

use regex::{Regex, Error as RegexError};
use string_cache::DefaultAtom as Atom;

use crate::lib;

struct Rule {
    color: Atom,
    colors_contained: Vec<(Atom, u32)>,
}

impl Rule {
}

impl FromStr for Rule {
    type Err = RegexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        fn split_colors(s: &str) -> Vec<(Atom, u32)> {
            if s == "no other bags" {
                vec![]
            } else {
                s.split(", ")
                    .map(|c| {
                        lazy_static! {
                            static ref RE: Regex = Regex::new(
                                r"^([0-9]+) ([a-z]+ [a-z]+) bags?$").unwrap();
                        }
                        let caps = RE.captures(c).expect("bad color contains list");
                        (Atom::from(&caps[2]), caps[1].parse::<u32>().expect("bad input qty"))
                    })
                    .collect()
            }
        }

        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^([a-z]+ [a-z]+) bags contain ([ ,a-z0-9]+)\.$").unwrap();
        }
        if let Some(caps) = RE.captures(s) {
            Ok(Self{ color: Atom::from(&caps[1]), colors_contained: split_colors(&caps[2]) })
        } else {
            Err(RegexError::Syntax(String::from("malformed input line")))
        }
    }

}

pub fn run(part: u32) {
    let rules: Vec<Rule> = lib::parse::read_to_vec::<Rule>("input/07.txt");

    if part == 1 {
        let contains: HashMap<_, _> = rules.iter()
            .map(|r| (r.color.clone(), r.colors_contained.iter().map(|v| v.0.clone()).collect()))
            .collect();
        let within = lib::algo::invert_hashmap(contains);
        let mut seen = HashSet::new();
        let mut q = VecDeque::new();
        q.push_back(Atom::from("shiny gold"));
        while let Some(color) = q.pop_front() {
            if seen.contains(&color) { continue; }
            q.extend(if let Some(colors) = within.get(&color) { colors.clone() } else { vec![] });
            seen.insert(color);
        }
        println!("number of colors that can contain shiny gold: {}", seen.len() - 1);
    } else if part == 2 {
        let contains_qty: HashMap<_, _> = rules.iter()
            .map(|r| (r.color.clone(), r.colors_contained.clone()))
            .collect();
        let mut total_bags = 0;
        let mut q = VecDeque::new();
        q.push_back((Atom::from("shiny gold"), 1));
        while let Some((color, qty)) = q.pop_front() {
            q.extend(
                if let Some(colors_qtys) = contains_qty.get(&color) {
                    colors_qtys.iter()
                        .map(|(c, q)| (c.clone(), q * qty))
                        .collect()
                } else {
                    vec![]
                });
            total_bags += qty;
        }
        println!("number of bags inside a shiny gold: {}", total_bags - 1);
    }
}
