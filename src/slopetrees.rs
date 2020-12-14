

use crate::lib;

pub fn run(part: u32) {
    let lines = lib::parse::read_to_vec::<String>("input/03.txt");
    let mut trees = 0;
    let mut x = 0;
    for row in lines.iter() {
        if row.chars().nth(x).unwrap() == '#' {
            trees += 1;
        }
        x = (x + 3) % row.len();
    }
    println!("num trees: {}", trees);
}
