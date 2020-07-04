//! [Problem 3](https://projecteuler.net/problem=3) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use prime::{Factorize, PrimeSet};

fn compute(n: u64) -> u64 {
    let ps = PrimeSet::new();
    n.factorize(&ps).map(|(base, _exp)| base).max().unwrap()
}

fn solve() -> String {
    compute(600851475143).to_string()
}
common::problem!("6857", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn factorize_13195() {
        assert_eq!(29, super::compute(13195));
    }
}
