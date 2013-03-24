use common::calc::{ each_fib };

use std::bigint::{ BigUint };

pub fn solve() -> ~str {
    let mut i = 0u;
    let limit = BigUint::from_str_radix(
        str::from_bytes(vec::from_elem(999, '9' as u8)), 10).get();
    for each_fib |f: &BigUint| {
        i += 1;
        if *f > limit { break; }
    }

    return i.to_str();
}