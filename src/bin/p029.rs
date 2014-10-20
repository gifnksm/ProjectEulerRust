#![warn(unused, bad_style,
        unnecessary_qualification, unnecessary_typecast, unused_result)]

extern crate common;
extern crate prime;

use std::collections::HashSet;
use common::Solver;
use prime::{Factorize, PrimeSet};

fn compute(a_max: uint, b_max: uint) -> uint {
    let mut set = HashSet::new();
    let ps = PrimeSet::new();

    for a in range(2u, a_max + 1) {
        let a_factor = a.factorize(&ps).collect::<Vec<(uint, int)>>();
        for b in range(2u, b_max + 1) {
            let ab_factor = a_factor
                .iter()
                .map(|&(base, exp)| (base, (exp) as uint * b))
                .collect::<Vec<(uint, uint)>>();
            set.insert(ab_factor);
        }
    }
    set.len()
}

fn solve() -> String {
    compute(100, 100).to_string()
}

fn main() { Solver::new("9183", solve).run(); }

#[cfg(test)]
mod tests {
    #[test]
    fn five() {
        assert_eq!(15, super::compute(5, 5));
    }
}
