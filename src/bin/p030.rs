#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(slicing_syntax)]

extern crate common;
extern crate iter;
extern crate integer;

use std::num;
use std::iter::AdditiveIterator;
use common::Solver;
use iter::CombinationOverlap;
use integer::Integer;

// 9^5     = 59049
// 9999    => 9^5 * 4 = 236196
// 99999   => 9^5 * 5 = 295245
// 999999  => 9^5 * 6 = 354294
// 9999999 => 9^5 * 7 = 413343

fn compute(len: uint, pow: uint) -> uint {
    let pows = Vec::from_fn(10, |i| num::pow(i, pow));
    let digits = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let mut sum = 0;
    for comb in CombinationOverlap::new(digits, len) {
        let num = comb.iter().map(|&e| pows[e]).sum();
        let mut ds = num.into_digits(10).collect::<Vec<_>>();
        ds.sort_by(|a, b| a.cmp(b));

        let zero_len = len - ds.len();
        if comb[zero_len ..] == ds[] && comb[.. zero_len].iter().all(|&x| x == 0) {
            sum += num
        }
    }
    sum - 1
}

fn solve() -> String {
    compute(6, 5).to_string()
}

fn main() { Solver::new("443839", solve).run(); }

#[cfg(test)]
mod tests {
    #[test]
    fn four() {
        assert_eq!(19316, super::compute(5, 4));
    }
}
