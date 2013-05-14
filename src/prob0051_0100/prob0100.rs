use core::from_str::{ FromStr };
use core::util;
use core::num::{ One };

use std::bigint::{ BigUint };

use common::calc;
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 100,
    answer: "756872327473",
    solver: solve
};

// b/s * (b-1)/(s-1) = 1/2
// 2b(b - 1) = s * (s-1)
// 2b^2 - 2b = s^2 - s
// 2(b - 1/2)^2 - 1/2 = (s - 1/2)^2 - 1/4
// 2(2b - 1)^2 - 2 = (2s - 1)^2 - 1
// (2s - 1)^2 - 2(2b - 1)^2 = -1
// x^2 - 2y = -1
// s = (x + 1) / 2
// b = (y + 1) / 2
fn solve() -> ~str {
    let one = One::one();
    let limit = FromStr::from_str("1000000000000").get();
    for calc::each_pel_neg::<BigUint>(2) |&x, &y| {
        if x.is_even() || y.is_even() { loop; }
        let b = (y + one) >> 1;
        let s = (x + one) >> 1;
        if s > limit { return b.to_str(); }
    }
    util::unreachable();
}
