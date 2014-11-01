#![warn(unused, bad_style,
        unused_qualifications, unused_typecasts, unused_results)]

extern crate common;
extern crate prime;

use std::iter;
use common::Solver;
use prime::{Factorized, PrimeSet};

fn combination(n: u64, r: u64) -> u64 {
    let ps = PrimeSet::new();
    let mut fac = Factorized::<u64>::new(&ps);
    for n in iter::range_inclusive(r + 1, n) {
        fac.mul_assign(n);
    }
    for n in iter::range_inclusive(1, n - r) {
        fac.div_assign(n);
    }
    fac.into_integer()
}

fn compute(w: u64, h: u64) -> u64 {
    combination(w + h, w)
}

fn solve() -> String { compute(20, 20).to_string() }

fn main() { Solver::new("137846528820", solve).run(); }

#[cfg(test)]
mod tests {
    #[test]
    fn route_2x2() {
        assert_eq!(6, super::compute(2, 2));
    }
}
