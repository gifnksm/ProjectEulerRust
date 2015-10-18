//! [Problem 7](https://projecteuler.net/problem=7) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results)]

#[macro_use(problem)]
extern crate common;
extern crate prime;

use prime::PrimeSet;

fn compute(n: usize) -> u64 {
    PrimeSet::new().nth(n)
}

fn solve() -> String {
    compute(10001 - 1).to_string()
}

problem!("104743", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn sixth_prime() {
        assert_eq!(13, super::compute(6 - 1));
    }
}
