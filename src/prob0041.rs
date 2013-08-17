#[link(name = "prob0041", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;
extern mod math;

use common::calc;
use math::prime;

pub static EXPECTED_ANSWER: &'static str = "7652413";

pub fn solve() -> ~str {
    let mut ans = 0;
    do calc::permutate_num(&[7, 6, 5, 4, 3, 2, 1], 7, 0, 9999999) |num, _rest| {
        if prime::contains(num) {
            ans = num;
            false
        } else {
            true
        }
    };
    return ans.to_str();
}
