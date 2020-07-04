//! [Problem 63](https://projecteuler.net/problem=63) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use num_bigint::BigUint;
use num_traits::FromPrimitive;

fn solve() -> String {
    let mut cnt = 1; // a == 1
    for a in (2usize..10).filter_map::<BigUint, _>(FromPrimitive::from_usize) {
        let mut n = 0;
        let mut an: BigUint = FromPrimitive::from_u32(1).unwrap();
        loop {
            n += 1;
            an *= &a;
            let an_str = an.to_string();
            if an_str.len() != n {
                break;
            }

            cnt += 1
        }
    }

    cnt.to_string()
}

common::problem!("49", solve);
