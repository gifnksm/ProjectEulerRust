//! [Problem 12](https://projecteuler.net/problem=12) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use prime::{Factorize, PrimeSet};
use seq::TriangularNums;

fn compute(limit: u64) -> u64 {
    let ps = PrimeSet::new();

    TriangularNums::<u64>::new()
        .find(|t| t.num_of_divisor(&ps) > limit)
        .unwrap()
}

fn solve() -> String {
    compute(500).to_string()
}

common::problem!("76576500", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn five_divisors() {
        assert_eq!(28, super::compute(5));
    }
}
