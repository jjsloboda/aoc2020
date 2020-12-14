

use crate::lib;

fn check_slope(lines: &Vec<String>, x_diff: usize, y_diff: usize) -> u64 {
    let mut trees = 0;
    let mut x = 0;
    for (y, row) in lines.iter().enumerate() {
        if y % y_diff != 0 {
            continue;
        }
        if row.chars().nth(x).unwrap() == '#' {
            trees += 1;
        }
        x = (x + x_diff) % row.len();
    }
    trees
}

pub fn run(part: u32) {
    let lines = lib::parse::read_to_vec::<String>("input/03.txt");
    if part == 1 {
        let trees = check_slope(&lines, 3, 1);
        println!("num trees: {}", trees);
    } else {
        let combos = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        let product: u64 = combos.iter().map(|&(xd, yd)| check_slope(&lines, xd, yd)).product();
        println!("product of trees: {}", product);
    }
}
