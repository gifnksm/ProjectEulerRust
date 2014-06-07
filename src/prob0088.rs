#![crate_id = "prob0088"]
#![crate_type = "rlib"]


use std::{iter, uint};
use std::iter::AdditiveIterator;
use std::collections::HashSet;

pub static EXPECTED_ANSWER: &'static str = "7587457";

fn each_sum_product(start: uint, end: uint, f: &mut |uint, uint, uint|) {
    sub(start, end, 0, 1, 0, f);

    fn sub(start: uint, end: uint, sum: uint, prod: uint, len: uint, f: &mut |uint, uint, uint|) {
        for n in iter::range_inclusive(start, end / prod) {
            if len > 0 { (*f)(sum + n, prod * n, len + 1) }
            sub(n, end, sum + n, prod * n, len + 1, f)
        }
    }
}

pub fn solve() -> String {
    let limit = 12000;

    let start = 2;
    let mut end = 4;
    let mut cnt = limit - 1;
    let mut nums = Vec::from_elem(limit + 1, uint::MAX);

    while cnt > 0 {
        each_sum_product(start, end, &mut |sum, prod, len| {
            let k = prod - sum + len;
            if k <= limit && prod < *nums.get(k) {
                if *nums.get(k) == uint::MAX { cnt -= 1; }
                *nums.get_mut(k) = prod;
            }
        });
        end *= 2;
    }

    let mut set = HashSet::new();
    for &n in nums.iter() {
        if n != uint::MAX { set.insert(n); }
    }

    return set.move_iter().sum().to_str();
}
