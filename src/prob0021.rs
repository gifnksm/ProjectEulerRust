#[link(name = "prob0021", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::vec;
use std::iterator::AdditiveIterator;
use common::prime;

pub static expected_answer: &'static str = "31626";

pub fn solve() -> ~str {
    let limit = 10000;

    let sum_of_divs = vec::from_fn(limit, |n| prime::sum_of_proper_divisors(n));
    let is_deficient = |&(n, div): &(uint, uint)| div < n;
    let is_amicable  = |&(n, div): &(uint, uint)| sum_of_divs[div] == n;
    return sum_of_divs.iter()
        .transform(|&n| n)
        .enumerate()
        .filter(is_deficient)
        .filter(is_amicable)
        .transform(|(a, b)| a + b)
        .sum()
        .to_str();
}
