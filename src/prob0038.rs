#[crate_id = "prob0038"];
#[crate_type = "rlib"];

extern crate common;
extern crate math;

use common::calc;
use math::numconv;

pub static EXPECTED_ANSWER: &'static str = "932718654";

pub fn solve() -> ~str {
    let mut ans = 0;
    calc::permutate_num([9, 8, 7, 6, 5, 4, 3, 2, 1], 4, 0, 9999, |num, rest| {
            let mut ds = numconv::to_digits(num * 2, 10).to_owned_vec();
            ds.sort_by(|a, b| b.cmp(a));

            if ds.as_slice() == rest {
                ans = num;
                false
            } else {
                true
            }
        });

    return format!("{}{}", ans, ans* 2);
}
