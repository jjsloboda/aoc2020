pub mod parse {

use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
//use std::ops::Try;
use std::str::FromStr;

pub fn read_to_vec<T>(filename: &str) -> Vec<T>
        where T: FromStr, T::Err: Debug {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.unwrap().parse::<T>().unwrap()).collect()
}

}

pub mod algo {

use std::collections::HashMap;
use std::hash::Hash;

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

use std::fmt::Debug;

pub fn invert_hashmap<T: Eq + Clone + Hash + Debug>(hm: HashMap<T, Vec<T>>) -> HashMap<T, Vec<T>> {
    let mut nhm: HashMap<T, Vec<T>> = HashMap::new();
    for (k, vv) in hm.iter() {
        for v in vv.iter() {
            let e = nhm.entry((*v).clone()).or_insert(vec![]);
            (*e).push(k.clone());
        }
    }
    nhm
}

}

pub mod ops {

pub fn xor(a: bool, b: bool) -> bool {
    a != b
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

    #[test]
    fn xor_test() {
        use super::ops::xor;

        assert_eq!(xor(false, false), false);
        assert_eq!(xor(false, true), true);
        assert_eq!(xor(true, false), true);
        assert_eq!(xor(true, true), false);
    }

}
