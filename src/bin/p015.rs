#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(phase)]

#[phase(plugin, link)] extern crate common;
extern crate prime;

use std::iter;
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

problem!("137846528820", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn route_2x2() {
        assert_eq!(6, super::compute(2, 2));
    }
}
