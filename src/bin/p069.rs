#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

extern crate common;
extern crate prime;

use common::Solver;
use prime::PrimeSet;

fn compute(limit: u64) -> u64 {
    let ps = PrimeSet::new();
    let mut n = 1;
    for p in ps.iter() {
        if n * p > limit { break }
        n *= p
    }
    n
}

fn solve() -> String {
    compute(1000000).to_string()
}

fn main() { Solver::new("510510", solve).run(); }

#[cfg(test)]
mod tests {
    #[test]
    fn ten() { assert_eq!(6, super::compute(10)); }
}
