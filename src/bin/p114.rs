//! [Problem 114](https://projecteuler.net/problem=114) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results)]

#![feature(range_inclusive)]

#[macro_use(problem)] extern crate common;

use std::iter;
use std::collections::HashMap;

fn get_cnt((n, m): (u32, u32), map: &mut HashMap<(u32, u32), u64>) -> u64 {
    if let Some(&x) = map.get(&(n, m)) {
        return x
    }

    if n < m { let _ = map.insert((n, m), 1); return 1; }

    let mut sum = 0;

    for len in iter::range_inclusive(m, n) { // most left red block length
        for i in iter::range_inclusive(0, n - len) { // most left red block position
            if n > len + i {
                sum += get_cnt((n - (len + i) - 1, m), map); // red block and black block
            } else {
                sum += 1;
            }
        }
    }
    sum += 1; // all black block
    let _ = map.insert((n, m), sum);

    sum
}

fn solve() -> String {
    let mut map = HashMap::new();
    get_cnt((50, 3), &mut map).to_string()
}

problem!("16475640049", solve);

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::get_cnt;

    #[test]
    fn small_len() {
        let mut map = HashMap::new();
        assert_eq!(1, get_cnt((1, 3), &mut map));
        assert_eq!(1, get_cnt((2, 3), &mut map));
        assert_eq!(2, get_cnt((3, 3), &mut map));
        assert_eq!(4, get_cnt((4, 3), &mut map));
        assert_eq!(17, get_cnt((7, 3), &mut map));
    }
}
