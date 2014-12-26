#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(phase)]

#[phase(plugin, link)] extern crate common;

use std::iter;
use std::collections::HashMap;

fn get_cnt(n: uint, m: uint, map: &mut HashMap<(uint, uint), uint>) -> uint {
    let mut sum = 0;
    match map.get(&(n, m)) {
        Some(&x) => return x,
        None     => {}
    }

    if n < m { let _ = map.insert((n, m), 1); return 1; }

    for len in iter::range_inclusive(m, n) { // most left red block length
        for i in iter::range_inclusive(0, n - len) { // most left red block position
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
    get_cnt(50, 3, &mut map).to_string()
}

problem!("16475640049", solve);

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::get_cnt;

    #[test]
    fn small_len() {
        let mut map = HashMap::new();
        assert_eq!(1, get_cnt(1, 3, &mut map));
        assert_eq!(1, get_cnt(2, 3, &mut map));
        assert_eq!(2, get_cnt(3, 3, &mut map));
        assert_eq!(4, get_cnt(4, 3, &mut map));
        assert_eq!(17, get_cnt(7, 3, &mut map));
    }
}
