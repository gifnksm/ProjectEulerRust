#![warn(unused, bad_style,
        unnecessary_qualification, unnecessary_typecast, unused_result)]

extern crate common;
extern crate prime;

use common::Solver;
use prime::{PrimeSet, Factorized};

fn compute(n: uint) -> uint {
    let ps = PrimeSet::new();
    let mut fac = Factorized::new(&ps);
    for i in range(1u, n) {
        fac.lcm_with(i);
    }
    fac.into_integer()
}

fn solve() -> String { compute(20).to_string() }

fn main() { Solver::new("232792560", solve).run(); }

#[cfg(test)]
mod tests {
    #[test]
    fn evenly_dividable_below_10() {
        assert_eq!(2520 , super::compute(10));
    }
}
