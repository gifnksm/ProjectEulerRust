extern mod euler;

use euler::extnum::{ one, from_uint };
use euler::biguint::{ BigUint };

fn main() {
    let mut f = one::<BigUint>();
    for 100.timesi |i| {
        let n = i + 1;
        f *= from_uint(n);
    }
    let mut sum = 0;
    for f.to_str().each() |n| {
        sum += (n - '0' as u8) as uint;
    }
    io::println(fmt!("%u", sum));
}
