mod lib;

mod expenses {

use crate::lib::parse;
use std::collections::HashSet;

pub fn sum_to_target_set(filename: &str, target: i64) -> Option<i64> {
    let nums = parse::read_i64_to_vec(filename);
    let mut nums_seen = HashSet::new();
    for n in nums {
        let complement = target - n;
        if nums_seen.contains(&complement) {
            return Some(n * complement);
        }
        nums_seen.insert(n);
    }
    None
}

}

fn main() {
    if let Some(product) = expenses::sum_to_target_set("input/01.txt", 2020) {
        println!("part 1, product: {}", product);
    } else {
        println!("no product found");
    }
}
