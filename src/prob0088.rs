#[link(name = "prob0088", vers = "0.0")];
#[crate_type = "lib"];



use std::{uint, vec};
use std::iterator::AdditiveIterator;
use std::hashmap::HashSet;

pub static expected_answer: &'static str = "7587457";

fn each_sum_product(start: uint, end: uint, f: &fn(uint, uint, uint) -> bool) -> bool {
    return sub(start, end, 0, 1, 0, f);

    fn sub(start: uint, end: uint, sum: uint, prod: uint, len: uint,
           f: &fn(uint, uint, uint) -> bool) -> bool {
        for uint::range(start, end / prod + 1) |n| {
            if len > 0 {
                if !f(sum + n, prod * n, len + 1) { return false; }
            }

            if !sub(n, end, sum + n, prod * n, len + 1, f) { return false; }
        }
        return true;
    }
}

pub fn solve() -> ~str {
    let limit = 12000;

    let start = 2;
    let mut end = 4;
    let mut cnt = limit - 1;
    let mut nums = vec::from_elem(limit + 1, uint::max_value);

    while cnt > 0 {
        for each_sum_product(start, end) |sum, prod, len| {
            let k = prod - sum + len;
            if k <= limit && prod < nums[k] {
                if nums[k] == uint::max_value { cnt -= 1; }
                nums[k] = prod;
            }
        }
        end *= 2;
    }

    let mut set = HashSet::new();
    for nums.iter().advance |&n| {
        if n != uint::max_value { set.insert(n); }
    }

    return set.iter().transform(|&x| x).sum().to_str();
}
