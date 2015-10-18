//! [Problem 25](https://projecteuler.net/problem=25) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results)]

#[macro_use(problem)]
extern crate common;
extern crate num;
extern crate seq;

use std::iter;
use num::BigUint;
use seq::Fibonacci;

fn compute(limit_len: usize) -> usize {
    let limit = iter::repeat("9")
                    .take(limit_len - 1)
                    .collect::<String>()
                    .parse::<BigUint>()
                    .unwrap();
    Fibonacci::<BigUint>::new()
        .take_while(|n| *n <= limit)
        .count() + 1
}

fn solve() -> String {
    compute(1000).to_string()
}

problem!("4782", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn three() {
        assert_eq!(12, super::compute(3));
    }
}
