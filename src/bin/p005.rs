//! [Problem 5](https://projecteuler.net/problem=5) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use prime::{Factorized, PrimeSet};

fn compute(n: u32) -> u32 {
    let ps = PrimeSet::new();
    let mut fac = Factorized::new(&ps);
    for i in 1..n {
        fac.lcm_with(i);
    }
    fac.into_integer()
}

fn solve() -> String {
    compute(20).to_string()
}

common::problem!("232792560", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn evenly_dividable_below_10() {
        assert_eq!(2520, super::compute(10));
    }
}
