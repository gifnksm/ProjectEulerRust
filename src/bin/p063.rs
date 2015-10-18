//! [Problem 63](https://projecteuler.net/problem=63) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results)]

#[macro_use(problem)]
extern crate common;
extern crate num;

use num::{BigUint, FromPrimitive};

fn solve() -> String {
    let mut cnt = 1; // a == 1
    for a in (2usize..10).filter_map::<BigUint, _>(|a| FromPrimitive::from_usize(a)) {
        let mut n = 0;
        let mut an: BigUint = FromPrimitive::from_u32(1).unwrap();
        loop {
            n += 1;
            an = an * &a;
            let an_str = an.to_string();
            if an_str.len() != n {
                break;
            }

            cnt += 1
        }
    }

    cnt.to_string()
}

problem!("49", solve);
