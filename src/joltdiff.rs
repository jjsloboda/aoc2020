

use std::collections::HashMap;

use crate::lib;

fn diffs(nums: &Vec<i64>) -> Vec<i64> {
    let mut diffs = Vec::with_capacity(nums.len() - 1);
    for i in 0..(nums.len() - 1) {
        diffs.push(nums[i+1] - nums[i]);
    }
    diffs
}

fn count_diffs(step_diffs: &Vec<i64>) -> (i64, i64, i64) {
    let (mut d1, mut d2, mut d3) = (0, 0, 0);
    for i in step_diffs.iter() {
        match i {
            1 => d1 += 1,
            2 => d2 += 1,
            3 => d3 += 1,
            _ => panic!("bad gap: {}", i),
        }
    }
    (d1, d2, d3)
}

fn count_combos(memo: &mut HashMap<(i64, usize), u64>, step_diffs: &Vec<i64>, last: i64, index: usize) -> u64 {
    // dynamic programming with memoization
    if let Some(combos) = memo.get(&(last, index)) {
        *combos
    } else {
        let combos = if index == 0 {
            1
        } else if last + step_diffs[index-1] > 3 {
            count_combos(memo, step_diffs, step_diffs[index-1], index-1)
        } else {
            let count_with = count_combos(memo, step_diffs, step_diffs[index-1], index-1);
            let count_without = count_combos(memo, step_diffs, last + step_diffs[index-1], index-1);
            count_with + count_without
        };
        memo.insert((last, index), combos);
        combos
    }
}

pub fn run(part: u32) {
    let mut input = lib::parse::read_to_vec::<i64>("input/10.txt");
    input.push(0);
    input.sort_unstable();
    input.push(input.last().unwrap() + 3);
    let step_diffs = diffs(&input);
    if part == 1 {
        let (d1, d2, d3) = count_diffs(&step_diffs);
        println!("d1: {}, d2: {}, d3: {}, d1 * d3: {}", d1, d2, d3, d1 * d3);
    } else if part == 2 {
        let last = step_diffs.last().unwrap();
        let index = step_diffs.len() - 1;
        let combos: u64 = count_combos(&mut HashMap::new(), &step_diffs, *last, index);
        println!("total combos: {}", combos);
    }
}
