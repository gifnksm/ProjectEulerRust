#[link(name = "prob0055", vers = "0.0")];
#[crate_type = "lib"];

extern mod extra;
extern mod common;

use std::{str, vec, uint};
use std::from_str::{FromStr};
use extra::bigint::{BigUint};
use common::problem::{Problem};

pub static problem: Problem<'static> = Problem {
    id: 55,
    answer: "249",
    solver: solve
};

fn reverse(n: &BigUint) -> BigUint {
    let s = n.to_str();
    let rev = str::from_bytes(vec::reversed(s.as_bytes()));
    return FromStr::from_str(rev).get();
}

fn is_lychrel(n: uint) -> bool {
    let n = BigUint::from_uint(n);
    let mut sum = n + reverse(&n);
    for 50.times {
        let rev_sum = reverse(&sum);
        if rev_sum == sum { return false; }
        sum = sum + rev_sum;
    }
    return true;
}

pub fn solve() -> ~str {
    let mut cnt = 0u;
    for uint::range(1, 10001) |n| {
        if is_lychrel(n) { cnt += 1; }
    }
    return cnt.to_str();
}
