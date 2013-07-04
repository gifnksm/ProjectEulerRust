#[link(name = "prob0029", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::uint;
use std::hashmap::HashSet;
use common::prime;

pub static EXPECTED_ANSWER: &'static str = "9183";

pub fn solve() -> ~str {
    let mut set = HashSet::new();

    for uint::range(2, 101) |a| {
        let a_factor: ~[(uint, int)] = prime::factorize(a).collect();
        for uint::range(2, 101) |b| {
            let ab_factor = a_factor
                .map(|&(base, exp)| (base, (exp) as uint * b));
            set.insert(ab_factor);
        }
    }

    return set.len().to_str();
}
