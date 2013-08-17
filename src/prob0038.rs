#[link(name = "prob0038", vers = "0.0")];
#[crate_type = "lib"];

extern mod extra;
extern mod common;
extern mod math;

use extra::sort;
use common::calc;
use math::numconv;

pub static EXPECTED_ANSWER: &'static str = "932718654";

pub fn solve() -> ~str {
    let mut ans = 0;
    do calc::permutate_num([9, 8, 7, 6, 5, 4, 3, 2, 1], 4, 0, 9999) |num, rest| {
        let mut ds = numconv::to_digits(num * 2, 10).to_owned_vec();
        sort::quick_sort(ds, |a, b| a >= b);

        if ds.as_slice() == rest {
            ans = num;
            false
        } else {
            true
        }
    };

    return fmt!("%u%u", ans, ans* 2);
}
