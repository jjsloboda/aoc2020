

use crate::lib;

pub fn run(part: u32) {
    let lines: Vec<String> = lib::parse::read_to_vec::<String>("input/06.txt")
        .split(|s| s.is_empty())
        .map(|l| l.join(""))
        .collect();
    let total: u64 = lines.iter()
        .map(|l| {
            let mut chars = l.chars().collect::<Vec<_>>();
            chars.sort_unstable();
            chars.dedup();
            chars.len() as u64
        })
        .sum();
    println!("total questions with yes: {}", total);
}
