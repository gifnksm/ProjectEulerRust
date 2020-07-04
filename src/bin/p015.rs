//! [Problem 15](https://projecteuler.net/problem=15) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use prime::PrimeSet;

fn compute(w: u64, h: u64) -> u64 {
    PrimeSet::new().combination(w + h, w)
}

fn solve() -> String {
    compute(20, 20).to_string()
}

common::problem!("137846528820", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn route_2x2() {
        assert_eq!(6, super::compute(2, 2));
    }
}
