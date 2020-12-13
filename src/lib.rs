pub mod parse {

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_i64_to_vec(filename: &str) -> Vec<i64> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.unwrap().parse::<i64>().unwrap()).collect()
}

}

pub mod algo {

pub fn binary_search(v: &Vec<i64>, q: i64, l: usize, h: usize) -> bool {
    if v.is_empty() {
        false
    } else if l + 1 == h {
        v[l] == q
    } else {
        let m = l + (h - l) / 2;
        binary_search(v, q, l, m) || binary_search(v, q, m, h)
    }
}

}

#[cfg(test)]
mod tests {

    #[test]
    fn binary_search_tests() {
        use super::algo::binary_search;

        // not found
        assert!(!binary_search(&vec![], 42, 0, 0));
        assert!(!binary_search(&vec![43], 42, 0, 1));
        assert!(!binary_search(&vec![41, 43], 42, 0, 2));
        assert!(!binary_search(&vec![41, 43, 44], 42, 0, 3));
        assert!(!binary_search(&vec![41, 43, 44, 45], 42, 0, 4));

        // found
        assert!(binary_search(&vec![42], 42, 0, 1));
        assert!(binary_search(&vec![42], 42, 0, 1));
        assert!(binary_search(&vec![41, 42], 42, 0, 2));
        assert!(binary_search(&vec![42, 43], 42, 0, 2));
        assert!(binary_search(&vec![41, 42, 44], 42, 0, 3));
        assert!(binary_search(&vec![40, 41, 42, 44], 42, 0, 4));
        assert!(binary_search(&vec![41, 42, 43, 44], 42, 0, 4));
    }

}
