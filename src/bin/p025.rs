 #![warn(unused, bad_style,
        unnecessary_qualification, unnecessary_typecast, unused_result)]

#![feature(slicing_syntax)]

extern crate num;

extern crate common;
extern crate seq;

use num::BigUint;
use common::Solver;
use seq::Fibonacci;

fn compute(limit_len: uint) -> uint {
    let limit = from_str("9".repeat(limit_len - 1)[]).unwrap();
    Fibonacci::<BigUint>::new()
        .take_while(|n| *n <= limit)
        .count() + 1
}

fn solve() -> String {
    compute(1000).to_string()
}

fn main() { Solver::new("2783915460", solve).run(); }

#[cfg(test)]
mod tests {
    #[test]
    fn three() {
        assert_eq!(12, super::compute(3));
    }
}
