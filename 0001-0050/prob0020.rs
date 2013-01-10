use num::{One};

extern mod std;
use std::bigint::{BigUint};

fn main() {
    let mut f = One::one::<BigUint>();
    for uint::range(0, 100) |i| {
        let n = i + 1;
        f *= BigUint::from_uint(n);
    }
    let mut sum = 0;
    for f.to_str().each() |n| {
        sum += (n - '0' as u8) as uint;
    }
    io::println(fmt!("%u", sum));
}
