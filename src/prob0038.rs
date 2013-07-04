#[link(name = "prob0038", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;
extern mod extra;

use std::{vec, util};
use extra::sort;
use common::calc;

pub static expected_answer: &'static str = "932718654";

pub fn solve() -> ~str {
    for calc::permutate_num([9, 8, 7, 6, 5, 4, 3, 2, 1], 4, 0, 9999) |num, rest| {
        let mut ds = calc::num_to_digits(num * 2, 10);
        sort::quick_sort(ds, |a, b| a >= b);

        if ds.as_slice() == rest {
            return fmt!("%u%u", num, num * 2);
        }
    }

    util::unreachable();
}
