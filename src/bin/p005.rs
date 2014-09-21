#![warn(unused, bad_style,
        unnecessary_qualification, unnecessary_typecast, unused_result)]

extern crate common;
extern crate prime;

use std::{cmp, num};
use std::collections::HashMap;
use common::Solver;
use prime::{PrimeSet, Factorize};

fn compute(n: uint) -> uint {
    let mut map = HashMap::new();
    let ps = PrimeSet::new();

    for i in range(1u, n) {
        for (b, e) in i.factorize(&ps) {
            let _ = map.insert_or_update_with(b, e, |_, v| {
                *v = cmp::max(*v, e);
            });
        }
    }

    map.into_iter()
        .fold(1, |prod, (base, exp)| {
            if exp > 0 {
                prod * num::pow(base, exp as uint)
            } else {
                prod / num::pow(base, (-exp) as uint)
            }
        })
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
