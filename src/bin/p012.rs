#![warn(unused, bad_style,
        unused_qualifications, unused_typecasts, unused_results)]

extern crate common;
extern crate prime;
extern crate seq;

use common::Solver;
use prime::{PrimeSet, Factorize};
use seq::TriangularNums;

fn compute(limit: uint) -> uint {
    let ps = PrimeSet::new();

    TriangularNums::<uint>::new()
        .skip_while(|&t| t.num_of_divisor(&ps) <= limit)
        .next()
        .unwrap()
}

fn solve() -> String { compute(500).to_string() }

fn main() { Solver::new("76576500", solve).run(); }

#[cfg(test)]
mod tests {
    #[test]
    fn five_divisors() {
        assert_eq!(28, super::compute(5));
    }
}
