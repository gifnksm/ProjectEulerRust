//! [Problem 55](https://projecteuler.net/problem=55) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(phase, slicing_syntax)]

extern crate num;
#[phase(plugin, link)] extern crate common;

use std::str::FromStr;
use num::bigint::BigUint;

fn reverse(n: &BigUint) -> BigUint {
    let s = n.to_string();
    let rev = String::from_chars(s.chars().rev().collect::<Vec<char>>()[]);
    FromStr::from_str(rev[]).unwrap()
}

fn is_lychrel(n: uint, limit: uint) -> bool {
    let n: BigUint = FromPrimitive::from_uint(n).unwrap();
    let mut sum = &n + reverse(&n);
    for _ in range(0u, limit) {
        let rev_sum = reverse(&sum);
        if rev_sum == sum { return false }
        sum = sum + rev_sum;
    }
    true
}

fn compute(max: uint, limit: uint) -> uint {
    range(1, max + 1)
        .filter(|&n| is_lychrel(n, limit))
        .count()
}

fn solve() -> String {
    compute(10000, 50).to_string()
}

problem!("249", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn is_lychrel() {
        assert!(!super::is_lychrel(349, 50));
        assert!(super::is_lychrel(196, 50));
        assert!(super::is_lychrel(10677, 52));
        assert!(!super::is_lychrel(10677, 53));
        assert!(super::is_lychrel(4994, 50));
    }
}
