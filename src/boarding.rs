

use crate::lib;

pub fn run(part: u32) {
    let mut seat_ids: Vec<_> = lib::parse::read_to_vec::<String>("input/05.txt")
        .iter()
        .map(|s| {
            let b = s
                .replace("F", "0")
                .replace("B", "1")
                .replace("L", "0")
                .replace("R", "1");
            u64::from_str_radix(&b, 2).expect("couldn't parse binary")
        })
        .collect();
    if part == 1 {
        let greatest = seat_ids.iter().max().expect("no elements in input");
        println!("greatest boarding pass: {}", greatest);
    } else if part == 2 {
        seat_ids.sort();
        for i in 0..seat_ids.len()-1 {
            let (s1, s2) = (seat_ids[i], seat_ids[i+1]);
            if s2 - s1 == 2 {
                println!("my seat: {}", s1 + 1);
                return;
            }
        }
    }
}
