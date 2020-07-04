//! [Problem 10](https://projecteuler.net/problem=10) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use prime::PrimeSet;

fn compute(limit: u64) -> u64 {
    let ps = PrimeSet::new();
    ps.iter().take_while(|&p| p < limit).sum()
}

fn solve() -> String {
    compute(2000000).to_string()
}

common::problem!("142913828922", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn four_seq() {
        assert_eq!(17, super::compute(10));
    }
}
