//! [Problem 47](https://projecteuler.net/problem=47) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(phase)]

#[phase(plugin, link)] extern crate common;
extern crate prime;

use std::iter;
use prime::{Factorize, PrimeSet};

fn compute(len: uint, num_factor: uint) -> uint {
    let ps = PrimeSet::new();
    let mut cnt = 0;

    for n in iter::count(1, 1) {
        if n.factorize(&ps).count() != num_factor {
            cnt = 0;
            continue
        }

        cnt += 1;
        if cnt == len {
            return n + 1 - len
        }
    }

    unreachable!()
}

fn solve() -> String {
    compute(4, 4).to_string()
}

problem!("134043", solve);

#[cfg(test)]
mod tests {
    #[test] fn two() { assert_eq!(14, super::compute(2, 2)); }
    #[test] fn three() { assert_eq!(644, super::compute(3, 3)); }
}
