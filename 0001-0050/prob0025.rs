extern mod euler;

use euler::calc::{ each_fib };

extern mod std;
use std::bigint::{ BigUint };

fn main() {
    let mut i = 0;
    let limit = BigUint::from_str_radix(
        str::from_bytes(vec::from_elem(999, '9' as u8)), 10).get();
    for each_fib |f: &BigUint| {
        i += 1;
        if *f > limit { break; }
    }

    io::println(fmt!("%d", i));
}