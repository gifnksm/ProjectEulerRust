#![crate_id = "prob0100"]
#![crate_type = "rlib"]

extern crate num;
extern crate math;

use std::from_str::FromStr;
use std::num::One;
use num::Integer;
use num::bigint::BigUint;
use math::cont_frac::PelNegIterator;

pub static EXPECTED_ANSWER: &'static str = "756872327473";

// b/s * (b-1)/(s-1) = 1/2
// 2b(b - 1) = s * (s-1)
// 2b^2 - 2b = s^2 - s
// 2(b - 1/2)^2 - 1/2 = (s - 1/2)^2 - 1/4
// 2(2b - 1)^2 - 2 = (2s - 1)^2 - 1
// (2s - 1)^2 - 2(2b - 1)^2 = -1
// x^2 - 2y = -1
// s = (x + 1) / 2
// b = (y + 1) / 2
pub fn solve() -> ~str {
    let one   = One::one();
    let limit = FromStr::from_str("1000000000000").unwrap();

    PelNegIterator::<BigUint>::new(2)
        .filter(|&(ref x, ref y)| x.is_odd() && y.is_odd())
        .map(|(x, y)| ((x + one) >> 1, (y + one) >> 1))
        .find(|&(ref x, ref _y)| ((*x) > limit))
        .map(|(_x, y)| y)
        .unwrap()
        .to_str()
}
