
use std::collections::HashSet;
use std::iter::FromIterator;

use crate::lib;

pub fn run(part: u32) {
    let input = lib::parse::read_to_vec::<i64>("input/09.txt");
    let mut preamble = HashSet::<&i64>::from_iter(&input[..25]);
    for i in 25..input.len() {
        let n = &input[i];
        let mut has_sum = false;
        for j in 0..25 {
            let complement = n - input[i-j];
            if preamble.contains(&complement) {
                has_sum = true;
                break;
            }
        }
        if !has_sum {
            println!("first number with no sum in preamble: {}", n);
        }
        preamble.remove(&input[i - 25]);
        preamble.insert(&input[i]);
    }
}
