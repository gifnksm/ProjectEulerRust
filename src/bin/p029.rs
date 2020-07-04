//! [Problem 29](https://projecteuler.net/problem=29) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use prime::{Factorize, PrimeSet};
use std::collections::HashSet;

fn compute(a_max: u32, b_max: u32) -> u32 {
    let mut set = HashSet::new();
    let ps = PrimeSet::new();

    for a in 2..(a_max + 1) {
        let a_factor = a.factorize(&ps).collect::<Vec<_>>();
        for b in 2..(b_max + 1) {
            let ab_factor = a_factor
                .iter()
                .map(|&(base, exp)| (base, (exp) as u32 * b))
                .collect::<Vec<_>>();
            let _ = set.insert(ab_factor);
        }
    }
    set.len() as u32
}

fn solve() -> String {
    compute(100, 100).to_string()
}

common::problem!("9183", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn five() {
        assert_eq!(15, super::compute(5, 5));
    }
}
