
use std::collections::{HashSet, HashMap, VecDeque};

use regex::Regex;
use string_cache::DefaultAtom as Atom;

use crate::lib;

struct Rule {
    color: Atom,
    colors_contained: Vec<Atom>,
}

impl Rule {
    pub fn from(s: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^([a-z]+ [a-z]+) bags contain ([ ,a-z0-9]+)\.$").unwrap();
        }
        let caps = RE.captures(s).expect("malformed input");
        Self{ color: Atom::from(&caps[1]), colors_contained: Rule::split_colors(&caps[2]) }
    }

    fn split_colors(s: &str) -> Vec<Atom> {
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
                    Atom::from(&caps[2])
                })
                .collect()
        }
    }
}

pub fn run(part: u32) {
    let rules: Vec<Rule> = lib::parse::read_to_vec::<String>("input/07.txt")
        .iter()
        .map(|l| Rule::from(l))
        .collect();
    let contains: HashMap<_, _> = rules.iter()
        .map(|r| (r.color.clone(), r.colors_contained.clone()))
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
}
