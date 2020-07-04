//! [Problem 55](https://projecteuler.net/problem=55) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use num_bigint::BigUint;
use num_traits::FromPrimitive;
use std::str::FromStr;

fn reverse(n: &BigUint) -> BigUint {
    let s = n.to_string();
    let rev = s.chars().rev().collect::<String>();
    FromStr::from_str(&rev).unwrap()
}

fn is_lychrel(n: u32, limit: usize) -> bool {
    let n: BigUint = FromPrimitive::from_u32(n).unwrap();
    let mut sum = &n + reverse(&n);
    for _ in 0..limit {
        let rev_sum = reverse(&sum);
        if rev_sum == sum {
            return false;
        }
        sum += rev_sum;
    }
    true
}

fn compute(max: u32, limit: usize) -> usize {
    (1..(max + 1)).filter(|&n| is_lychrel(n, limit)).count()
}

fn solve() -> String {
    compute(10000, 50).to_string()
}

common::problem!("249", solve);

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
