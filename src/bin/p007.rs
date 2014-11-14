#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

extern crate common;
extern crate prime;

use common::Solver;
use prime::PrimeSet;

fn compute(n: uint) -> u64 { PrimeSet::new().nth(n) }

fn solve() -> String { compute(10001 - 1).to_string() }

fn main() { Solver::new("104743", solve).run(); }

#[cfg(test)]
mod tests {
    #[test]
    fn sixth_prime() {
        assert_eq!(13 , super::compute(6 - 1));
    }
}
