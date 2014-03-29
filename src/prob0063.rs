#![crate_id = "prob0063"]
#![crate_id = "prob0063"]
#![crate_type = "rlib"]
#![crate_type = "rlib"]

extern crate num;

use num::bigint::BigUint;

pub static EXPECTED_ANSWER: &'static str = "49";

pub fn solve() -> ~str {
    let mut cnt = 1u; // a == 1
    for a in range::<BigUint>(FromPrimitive::from_uint(2).unwrap(),
                              FromPrimitive::from_uint(10).unwrap()) {
        let mut n = 0;
        let mut an: BigUint = FromPrimitive::from_uint(1).unwrap();
        loop {
            n += 1;
            an = an * a;
            let an_str = an.to_str();
            if an_str.len() != n { break; }

            cnt += 1;
        }
    }

    return cnt.to_str();
}

