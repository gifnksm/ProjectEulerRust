#![crate_name = "prob0055"]
#![crate_type = "rlib"]

extern crate num;

use std::from_str::FromStr;
use std::num::FromPrimitive;
use num::bigint::BigUint;

pub const EXPECTED_ANSWER: &'static str = "249";

fn reverse(n: &BigUint) -> BigUint {
    let s = n.to_string();
    let rev = String::from_chars(s.as_slice().chars().rev().collect::<Vec<char>>().as_slice());
    FromStr::from_str(rev.as_slice()).unwrap()
}

fn is_lychrel(n: uint) -> bool {
    let n: BigUint = FromPrimitive::from_uint(n).unwrap();
    let mut sum = n + reverse(&n);
    for _ in range(0u, 50) {
        let rev_sum = reverse(&sum);
        if rev_sum == sum { return false; }
        sum = sum + rev_sum;
    }
    return true;
}

pub fn solve() -> String {
    let mut cnt = 0u;
    for n in range(1u, 10001) {
        if is_lychrel(n) { cnt += 1; }
    }
    cnt.to_string()
}
