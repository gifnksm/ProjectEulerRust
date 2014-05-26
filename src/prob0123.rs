#![crate_id = "prob0123"]
#![crate_type = "rlib"]

extern crate num;
extern crate math;

use std::iter;
use num::Integer;
use math::prime::Prime;

pub static EXPECTED_ANSWER: &'static str = "21035";

// from problem 120
// f(n) := (p[n]-1)^n + (p[n]+1)^n
//
// if n is even:
//   f(n) ≡ 1  f   mod p[n]^2
// if n is odd:
//   f(n) ≡ 2np[n] mod p[n]^2

fn get_mod(n: uint, pn: uint) -> uint {
    if n.is_even() {
        1
    } else {
        (2 * n * pn) % (pn * pn)
    }
}

pub fn solve() -> String {
    let limit = std::num::pow(10u, 10);

    let ps = Prime::new();
    iter::count(1, 1u)
        .zip(ps.iter())
        .find(|&(n, pn)| get_mod(n, pn) > limit)
        .unwrap()
        .val0()
        .to_str()
}
