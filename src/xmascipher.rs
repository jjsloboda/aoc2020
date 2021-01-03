
use std::collections::HashSet;
use std::iter::FromIterator;

use crate::lib;

fn find_invalid(nums: &Vec<i64>) -> Option<i64> {
    let mut preamble = HashSet::<&i64>::from_iter(&nums[..25]);
    for i in 25..nums.len() {
        let n = &nums[i];
        let mut has_sum = false;
        for j in 0..25 {
            let complement = n - nums[i-j];
            if preamble.contains(&complement) {
                has_sum = true;
                break;
            }
        }
        if !has_sum {
            return Some(*n);
        }
        preamble.remove(&nums[i - 25]);
        preamble.insert(&nums[i]);
    }
    None
}

fn find_summing_range(nums: &Vec<i64>, target: i64) -> Option<(usize, usize)> {
    let (mut imin, mut imax) = (0, 1);
    let mut total = nums[imin] + nums[imax];
    while imin < imax && imax < nums.len() {
        if total < target {
            imax += 1;
            total += nums[imax];
        } else if total > target {
            total -= nums[imin];
            imin += 1;
        } else {
            return Some((imin, imax));
        }
    }
    None
}

pub fn run(part: u32) {
    let input = lib::parse::read_to_vec::<i64>("input/09.txt");
    let invalid_num = find_invalid(&input).expect("no invalid num found");
    if part == 1 {
        println!("first number with no sum in preamble: {}", invalid_num);
    } else if part == 2 {
        let (i, j) = find_summing_range(&input, invalid_num).expect("no range found");
        let srange = &input[i..j+1];
        let (min, max) = (srange.iter().min().unwrap(), srange.iter().max().unwrap());
        println!("min: {}, max: {}, sum: {}", min, max, min + max);
    }
}
