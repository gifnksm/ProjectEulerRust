#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(phase)]

#[phase(plugin, link)] extern crate common;
extern crate prime;

use std::iter::{mod, AdditiveIterator};
use prime::PrimeSet;

fn compute(limit: u64) -> u64 {
    let prime = PrimeSet::new();

    let mut v = Vec::from_fn((limit as uint) + 1, |n| n as u64);
    v[1] = 0;

    for p in prime.iter() {
        if p > limit { break; }
        for n in iter::range_step(p, limit + 1, p) {
            v[n as uint] = (v[n as uint] * (p - 1)) / p;
        }
    }

    v.into_iter().sum()
}

fn solve() -> String {
    compute(1000000).to_string()
}

problem!("303963552391", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn eight() { assert_eq!(21, super::compute(8)); }
}
