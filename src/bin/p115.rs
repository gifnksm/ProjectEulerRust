//! [Problem 115](https://projecteuler.net/problem=115) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use std::collections::HashMap;

fn get_cnt(n: usize, m: usize, map: &mut HashMap<(usize, usize), usize>) -> usize {
    if let Some(&x) = map.get(&(n, m)) {
        return x;
    }

    if n < m {
        let _ = map.insert((n, m), 1);
        return 1;
    }

    let mut sum = 0;

    for len in m..(n + 1) {
        // most left red block length
        for i in 0..(n - len + 1) {
            // most left red block position
            if n > len + i {
                sum += get_cnt(n - (len + i) - 1, m, map); // red block and black block
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
    (1..)
        .find(|&n| get_cnt(n, 50, &mut map) > 1000000)
        .unwrap()
        .to_string()
}

common::problem!("168", solve);

#[cfg(test)]
mod tests {
    use super::get_cnt;
    use std::collections::HashMap;

    #[test]
    fn small_len() {
        let mut map = HashMap::new();
        assert_eq!(1, get_cnt(1, 3, &mut map));
        assert_eq!(1, get_cnt(2, 3, &mut map));
        assert_eq!(2, get_cnt(3, 3, &mut map));
        assert_eq!(4, get_cnt(4, 3, &mut map));
        assert_eq!(17, get_cnt(7, 3, &mut map));
        assert_eq!(673135, get_cnt(29, 3, &mut map));
        assert_eq!(1089155, get_cnt(30, 3, &mut map));
        assert_eq!(880711, get_cnt(56, 10, &mut map));
        assert_eq!(1148904, get_cnt(57, 10, &mut map));
    }
}
