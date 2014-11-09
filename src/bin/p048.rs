#![warn(unused, bad_style,
        unused_qualifications, unused_typecasts, unused_results)]

extern crate common;
extern crate integer;
extern crate num;

use common::Solver;
use num::BigUint;
use integer::Integer;

fn compute(max: uint, modulo: uint) -> uint {
    let bu_m: BigUint = FromPrimitive::from_uint(modulo).unwrap();

    let mut sum = 0;
    for n in range(1, max + 1) {
        let bu_n: BigUint = FromPrimitive::from_uint(n).unwrap();
        let pow = bu_n.mod_pow(&bu_n, &bu_m).to_uint().unwrap();
        sum = (sum + pow) % modulo;
    }
    sum
}

fn solve() -> String {
    compute(1000, 100_0000_0000).to_string()
}

fn main() { Solver::new("9110846700", solve).run(); }

#[cfg(test)]
mod tests {
    #[test]
    fn ten() {
        let modulo = 100_0000_0000;
        assert_eq!(10405071317 % modulo, super::compute(10, modulo))
    }
}
