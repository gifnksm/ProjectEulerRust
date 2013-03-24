use core::util::{ unreachable };

use euler::calc::{ permutate_num };
use euler::prime::{ Prime };

pub fn solve() -> uint {
    let mut ps = Prime();
    for permutate_num(&[7, 6, 5, 4, 3, 2, 1], 7, 0, 9999999) |num, _rest| {
        if ps.is_prime(num) {
            return num;
        }
    }

    unreachable();
}
