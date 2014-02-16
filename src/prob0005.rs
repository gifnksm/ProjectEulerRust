#[crate_id = "prob0005"];
#[crate_type = "rlib"];

extern crate data;
extern crate math;

use std::vec;
use data::monoid::{Max, MergeMultiMonoidIterator, Wrap};
use math::prime::{Prime, FactorIterator};

pub static EXPECTED_ANSWER: &'static str = "232792560";

pub fn solve() -> ~str {
    let prime = Prime::new();
    let fs = vec::from_fn(20, |i| {
            prime.factorize(i + 1).map(|(base, exp)| (base, Max(exp)))
        });
    let mut it = MergeMultiMonoidIterator::new(fs).map(|(base, m)| (base, m.unwrap()));
    return it.to_uint().to_str();
}
