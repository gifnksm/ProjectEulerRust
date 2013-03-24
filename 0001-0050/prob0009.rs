use core::util::{ unreachable };

use euler::arith::{ isqrt };

fn each_pyrhagorean(sum: uint, f: &fn(uint, uint, uint) -> bool) {
    for uint::range(2, sum - 2) |c| {
        for uint::range(1, uint::min((sum - c) / 2, isqrt(c*c / 2))) |a| {
            let b = sum - c - a;
            if a * a + b * b == c * c {
                if !f(a, b, c) { return; }
            }
        }
    }
}

pub fn solve() -> uint {
    for each_pyrhagorean(1000) |a, b, c| {
        return a * b * c;
    }

    unreachable();
}
