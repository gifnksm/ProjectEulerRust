extern mod euler;

use euler::calc::{ each_fib };
use euler::biguint::{ BigUint, from_str_radix };

fn main() {
    let mut i = 0;
    let limit = from_str_radix::<BigUint>(str::from_bytes(vec::from_elem(999, '9' as u8)), 10).get();
    for each_fib |f: &BigUint| {
        i += 1;
        if *f > limit { break; }
    }

    io::println(fmt!("%d", i));
}