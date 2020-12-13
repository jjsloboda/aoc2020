mod lib;

use crate::lib::parse;

mod expenses {

use std::collections::HashSet;

pub fn two_sum_to_target<'a, T: IntoIterator<Item=&'a i64>>(nums: T, target: i64) -> Option<i64> {
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

pub fn three_sum_to_target(nums: &Vec<i64>, target: i64) -> Option<i64> {
    if nums.len() >= 3 {
        let mut n = nums.clone();
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

}

fn main() {
    let nums = parse::read_i64_to_vec("input/01.txt");
    if let Some(product) = expenses::two_sum_to_target(&nums, 2020) {
        println!("part 1, product: {}", product);
    } else {
        println!("no product found");
    }
    if let Some(product) = expenses::three_sum_to_target(&nums, 2020) {
        println!("part 2, product: {}", product);
    } else {
        println!("no product found");
    }
}
