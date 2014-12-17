#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

extern crate common;
extern crate num;

use common::Solver;
use num::BigUint;

fn solve() -> String {
    let mut cnt = 1u; // a == 1
    for a in range(2, 10).filter_map::<BigUint, _>(|a| FromPrimitive::from_uint(a)) {
        let mut n = 0;
        let mut an: BigUint = FromPrimitive::from_uint(1).unwrap();
        loop {
            n += 1;
            an = an * &a;
            let an_str = an.to_string();
            if an_str.len() != n { break }

            cnt += 1
        }
    }

    cnt.to_string()
}

fn main() { Solver::new("49", solve).run(); }

