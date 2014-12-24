#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(slicing_syntax)]

extern crate num;

extern crate common;
extern crate seq;

use std::iter;
use num::BigUint;
use common::Solver;
use seq::Fibonacci;

fn compute(limit_len: uint) -> uint {
    let limit = iter::repeat("9").take(limit_len - 1).collect::<String>().parse::<BigUint>().unwrap();
    Fibonacci::<BigUint>::new()
        .take_while(|n| *n <= limit)
        .count() + 1
}

fn solve() -> String {
    compute(1000).to_string()
}

fn main() { Solver::new("4782", solve).run(); }

#[cfg(test)]
mod tests {
    #[test]
    fn three() {
        assert_eq!(12, super::compute(3));
    }
}
