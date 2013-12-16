#[crate_type = "rlib"];

extern mod common;
extern mod math;

use common::calc;
use math::prime;

pub static EXPECTED_ANSWER: &'static str = "7652413";

pub fn solve() -> ~str {
    let mut ans = 0;
    calc::permutate_num(&[7, 6, 5, 4, 3, 2, 1], 7, 0, 9999999, |num, _rest| {
        if prime::contains(num) {
            ans = num;
            false
        } else {
            true
        }
    });
    return ans.to_str();
}
