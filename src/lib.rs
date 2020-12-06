pub mod parse {

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_i64_to_vec(filename: &str) -> Vec<i64> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.unwrap().parse::<i64>().unwrap()).collect()
}

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
