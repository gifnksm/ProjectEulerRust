//! [Problem 117](https://projecteuler.net/problem=117) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use std::collections::HashMap;

fn count(len: u32, map: &mut HashMap<u32, u64>) -> u64 {
    if let Some(&x) = map.get(&len) {
        return x;
    }

    let mut sum = 0;
    for i in 0..(len + 1) {
        // most left block position
        if len - i >= 2 {
            sum += count(len - i - 2, map);
        } // red
        if len - i >= 3 {
            sum += count(len - i - 3, map);
        } // green
        if len - i >= 4 {
            sum += count(len - i - 4, map);
        } // blue
    }
    sum += 1; // all black
    let _ = map.insert(len, sum);
    sum
}

fn solve() -> String {
    let mut map = HashMap::new();
    count(50, &mut map).to_string()
}

common::problem!("100808458960497", solve);

#[cfg(test)]
mod tests {
    use super::count;
    use std::collections::HashMap;

    #[test]
    fn couunt_test() {
        let mut map = HashMap::new();
        assert_eq!(15, count(5, &mut map));
    }
}
