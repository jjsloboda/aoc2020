

use crate::lib;

pub fn run(part: u32) {
    let mut input = lib::parse::read_to_vec::<i64>("input/10.txt");
    input.push(0);
    input.sort_unstable();
    let (mut d1, mut d2, mut d3) = (0, 0, 1);
    for i in 0..(input.len() - 1) {
        match input[i+1] - input[i] {
            1 => d1 += 1,
            2 => d2 += 1,
            3 => d3 += 1,
            _ => panic!("bad gap: ith: {}, i+1th: {}", input[i], input[i+1]),
        }
    }
    println!("d1: {}, d2: {}, d3: {}, d1 * d3: {}", d1, d2, d3, d1 * d3);
}
