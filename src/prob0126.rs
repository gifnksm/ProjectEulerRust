#![crate_id = "prob0126"]
#![crate_id = "prob0126"]
#![crate_type = "rlib"]
#![crate_type = "rlib"]

use std::{iter, slice};

pub static EXPECTED_ANSWER: &'static str = "18522";

// cube size: (a, b, c)
// nth layer: 4(n-1)(n+a+b+c-2) + 2(ab+bc+ca)
fn f0(a: uint, b: uint, c: uint) -> uint { 2 * (a*b + b*c + c*a) }

pub fn solve() -> ~str {
    let sum   = 1000;
    let limit = 20000;
    let mut cnt = slice::from_elem(limit, 0u);

    for a in iter::count(1u, 1) {
        if f0(a, 1, 1) > limit { break; }

        for b in iter::range_inclusive(1, a) {
            if f0(a, b, 1) > limit { break; }

            for c in iter::range_inclusive(1, b) {
                let p = f0(a, b, c);
                if p > limit { break; }
                let q = a + b + c - 2;

                for n in iter::count(1u, 1) {
                    let f = 4*(n-1)*(n+q) + p;
                    if f >= cnt.len() { break; }
                    cnt[f] += 1;
                }
            }
        }
    }

    cnt.iter()
        .position(|&n| n == sum)
        .unwrap()
        .to_str()
}
