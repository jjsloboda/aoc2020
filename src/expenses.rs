use std::collections::HashSet;

use crate::lib;

pub fn run(part: u32) {
    let nums = lib::parse::read_to_vec::<i64>("input/01.txt");
    let func = if part == 1 { two_sum_to_target } else { three_sum_to_target };
    if let Some(product) = func(&nums, 2020) {
        println!("product: {}", product);
    } else {
        println!("no product found");
    }
}

fn two_sum_to_target<'a, T: IntoIterator<Item=&'a i64>>(nums: T, target: i64) -> Option<i64> {
    let mut nums_seen = HashSet::new();
    for n in nums.into_iter() {
        let complement = target - n;
        if nums_seen.contains(&complement) {
            return Some(n * complement);
        }
        nums_seen.insert(n);
    }
    None
}

fn three_sum_to_target<'a, T: IntoIterator<Item=&'a i64>>(nums: T, target: i64) -> Option<i64> {
    let mut n: Vec<i64> = nums.into_iter().cloned().collect();
    if n.len() >= 3 {
        n.sort();
        let (mut l, mut h) = (0, n.len() - 1);
        while l < h {
            if let Some(complement_product) = two_sum_to_target(&n[l..h], target - n[h]) {
                return Some(complement_product * n[h])
            } else {
                h -= 1;
            }
            if let Some(complement_product) = two_sum_to_target(&n[l+1..h+1], target - n[l]) {
                return Some(complement_product * n[l])
            } else {
                l += 1;
            }
        }
        None
    } else {
        None
    }
}
