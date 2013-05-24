#[link(name = "prob0038", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;
extern mod extra;

use std::util::{ unreachable };
use extra::sort::{ quick_sort };
use common::calc::{ permutate_num, num_to_digits };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 38,
    answer: "932718654",
    solver: solve
};

pub fn solve() -> ~str {
    for permutate_num([9, 8, 7, 6, 5, 4, 3, 2, 1], 4, 0, 9999) |num, rest| {
        let mut ds = num_to_digits(num * 2, 10);
        quick_sort(ds, |a, b| a >= b);

        if vec::eq(ds, rest) {
            return fmt!("%u%u", num, num * 2);
        }
    }

    unreachable();
}
