//! [Problem 116](https://projecteuler.net/problem=116) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use std::collections::HashMap;

fn count(len: u64, unit: u64, map: &mut HashMap<(u64, u64), u64>) -> u64 {
    if let Some(&x) = map.get(&(len, unit)) {
        return x;
    }

    if len < unit {
        let _ = map.insert((len, unit), 1);
        return 1;
    }

    let mut sum = 0;
    for i in 0..(len - unit + 1) {
        // most left block position
        sum += count(len - (unit + i), unit, map);
    }
    sum += 1;
    let _ = map.insert((len, unit), sum);
    sum
}

fn count_red(len: u64, map: &mut HashMap<(u64, u64), u64>) -> u64 {
    count(len, 2, map) - 1
}
fn count_green(len: u64, map: &mut HashMap<(u64, u64), u64>) -> u64 {
    count(len, 3, map) - 1
}
fn count_blue(len: u64, map: &mut HashMap<(u64, u64), u64>) -> u64 {
    count(len, 4, map) - 1
}
fn count_all(len: u64, map: &mut HashMap<(u64, u64), u64>) -> u64 {
    count_red(len, map) + count_green(len, map) + count_blue(len, map)
}

fn solve() -> String {
    let mut map = HashMap::new();
    count_all(50, &mut map).to_string()
}

common::problem!("20492570929", solve);

#[cfg(test)]
mod tests {
    use super::{count_all, count_blue, count_green, count_red};
    use std::collections::HashMap;

    #[test]
    fn count() {
        let mut map = HashMap::new();
        assert_eq!(7, count_red(5, &mut map));
        assert_eq!(3, count_green(5, &mut map));
        assert_eq!(2, count_blue(5, &mut map));
        assert_eq!(12, count_all(5, &mut map));
    }
}
