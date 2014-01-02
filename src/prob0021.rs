#[crate_type = "rlib"];

extern mod math;

use std::vec;
use std::iter::AdditiveIterator;
use math::prime::Prime;

pub static EXPECTED_ANSWER: &'static str = "31626";

pub fn solve() -> ~str {
    let limit = 10000;
    let prime = Prime::new();

    let sum_of_div = vec::from_fn(limit, |n| prime.sum_of_proper_divisor(n));
    let is_deficient = |&(n, div): &(uint, uint)| div < n;
    let is_amicable  = |&(n, div): &(uint, uint)| sum_of_div[div] == n;
    sum_of_div
        .iter()
        .map(|&n| n)
        .enumerate()
        .filter(is_deficient)
        .filter(is_amicable)
        .map(|(a, b)| a + b)
        .sum()
        .to_str()
}
