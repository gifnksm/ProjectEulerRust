#[link(name = "prob0005", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::vec;
use common::prime;
use common::monoid::{Max, MergeMultiMonoidIterator, Wrap};

pub static EXPECTED_ANSWER: &'static str = "232792560";

pub fn solve() -> ~str {
    let fs = do vec::from_fn(20) |i| { prime::factorize(i + 1) };
    let it = MergeMultiMonoidIterator::new(
        fs.map(|&x| x.transform(|(base, exp)| (base, Max(exp))))
    ).transform(|(base, m)| (base, m.unwrap()));
    return prime::factors_to_uint(it).to_str();
}
