//! [Problem 48](https://projecteuler.net/problem=48) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use num_bigint::BigUint;
use num_traits::{FromPrimitive, ToPrimitive};

fn compute(max: u64, modulo: u64) -> u64 {
    let bu_m: BigUint = FromPrimitive::from_u64(modulo).unwrap();

    let mut sum = 0;
    for n in 1..(max + 1) {
        let bu_n: BigUint = FromPrimitive::from_u64(n).unwrap();
        let pow = bu_n.modpow(&bu_n, &bu_m).to_u64().unwrap();
        sum = (sum + pow) % modulo;
    }
    sum
}

fn solve() -> String {
    compute(1000, 100_0000_0000).to_string()
}

common::problem!("9110846700", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn ten() {
        let modulo = 100_0000_0000;
        assert_eq!(10405071317 % modulo, super::compute(10, modulo))
    }
}
