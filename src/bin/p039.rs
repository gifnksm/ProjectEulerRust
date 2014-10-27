#![warn(unused, bad_style,
        unnecessary_qualification, unnecessary_typecast, unused_result)]

extern crate common;
extern crate integer;
extern crate seq;

use std::collections::HashMap;
use common::Solver;
use integer::Integer;
use seq::PrimitivePythagoreans;

fn num_sum_pythagorean(limit: uint) -> HashMap<uint, uint> {
    let mut map = HashMap::<uint, uint>::new();

    for m in range(1, ((1 + limit).sqrt() - 1) / 2) {
        for (a, b, c) in PrimitivePythagoreans::new(m) {
            let s = a + b + c;
            for k in range(1, limit / s + 1) {
                let new_val = map.find(&(k * s)).map_or(1, |&v| v + 1);
                map.insert(k * s, new_val);
            }
        }
    }

    map
}

fn compute(limit: uint) -> uint {
    let map = num_sum_pythagorean(limit);
    let (max_key, _max_val) = map.iter().max_by(|&(&_k, &v)| v).unwrap();
    *max_key
}

fn solve() -> String {
    compute(1000).to_string()
}

fn main() { Solver::new("840", solve).run(); }

#[cfg(test)]
mod tests {
    #[test]
    fn limit_120() {
        let map = super::num_sum_pythagorean(120);
        assert_eq!(3, *map.find(&120).unwrap());
    }
}
