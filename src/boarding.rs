

use crate::lib;

pub fn run(part: u32) {
    let greatest: u64 = lib::parse::read_to_vec::<String>("input/05.txt")
        .iter()
        .map(|s| {
            let b = s
                .replace("F", "0")
                .replace("B", "1")
                .replace("L", "0")
                .replace("R", "1");
            u64::from_str_radix(&b, 2).expect("couldn't parse binary")
        })
        .max()
        .expect("no elements in input");
    println!("greatest boarding pass: {}", greatest);
}
