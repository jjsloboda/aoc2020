
use std::collections::HashMap;

use counter::Counter;

use crate::lib;

pub fn run(part: u32) {
    let lines: Vec<String> = lib::parse::read_to_vec::<String>("input/06.txt")
        .split(|s| s.is_empty())
        .map(|l| l.join(" "))
        .collect();
    if part == 1 {
        let total: u64 = lines.iter()
            .map(|l| {
                let mut chars = l.chars().collect::<Vec<_>>();
                chars.sort_unstable();
                chars.dedup();
                (if l.contains(" ") { chars.len() - 1 } else { chars.len() }) as u64
            })
            .sum();
        println!("total questions with one yes: {}", total);
    } else if part == 2 {
        let total: u64 = lines.iter()
            .map(|l| {
                let counter = l.chars().collect::<Counter<_>>();
                let num_ppl = counter[&' '] + 1;
                counter.values()
                    .map(|&v| if v == num_ppl {1} else {0})
                    .sum::<u64>()
            })
            .sum();
        println!("total questions with all yes: {}", total);
    }
}
