//! [Problem 92](https://projecteuler.net/problem=92) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#[macro_use(problem)] extern crate common;
extern crate integer;

use std::iter::{self, AdditiveIterator};
use integer::Integer;

fn square_digit_sum(n: uint) -> uint {
    n.into_digits(10).map(|x| x * x).sum()
}

fn is_reach_89(n: uint, map: &mut [Option<bool>]) -> bool {
    if n >= map.len() {
        return is_reach_89(square_digit_sum(n), map)
    }

    if let Some(b) = map[n] {
        return b
    }

    let result = is_reach_89(square_digit_sum(n), map);
    map[n] = Some(result);
    result
}

fn create_map(limit: uint) -> Vec<Option<bool>> {
    let mut map = iter::repeat(None).take((limit - 1).to_string().len() * 81 + 1).collect::<Vec<_>>();
    map[0]  = Some(false);
    map[1]  = Some(false);
    map[89] = Some(true);
    map
}

fn solve() -> String {
    let limit = 10000000;
    let mut map = create_map(limit);
    range(1, limit)
        .filter(|&n| is_reach_89(n, map.as_mut_slice()))
        .count()
        .to_string()
}

problem!("8581146", solve);


#[cfg(test)]
mod tests {
    #[test]
    fn is_reach_89() {
        let mut map = super::create_map(100);
        assert!(!super::is_reach_89(44, map.as_mut_slice()));
        assert!(super::is_reach_89(85, map.as_mut_slice()));
    }
}
