use core::num::{ One };

use std::bigint::{ BigUint };

pub fn solve() -> uint {
    let mut f = One::one::<BigUint>();
    for uint::range(0, 100) |i| {
        f = f * BigUint::from_uint(i + 1);
    }
    let mut sum = 0;
    for f.to_str().each() |n| {
        sum += (n - '0' as u8) as uint;
    }
    return sum;
}
