//! [Problem 76](https://projecteuler.net/problem=76) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use std::collections::HashMap;

fn count_way(sum: u32) -> u32 {
    let mut map = HashMap::new();
    return count_sub(sum, 1, &mut map) - 1;

    fn count_sub(sum: u32, min_n: u32, map: &mut HashMap<(u32, u32), u32>) -> u32 {
        let mut cnt = 1; // only sum
        for k in min_n..(sum / 2 + 1) {
            let n = match map.get(&(sum - k, k)) {
                Some(&n) => n,
                None => count_sub(sum - k, k, map),
            };
            cnt += n;
        }
        let _ = map.insert((sum, min_n), cnt);
        cnt
    }
}

fn solve() -> String {
    count_way(100).to_string()
}

common::problem!("190569291", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn five() {
        assert_eq!(6, super::count_way(5));
    }
}
