#[link(name = "prob0047", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::{iterator, util};
use common::prime;

pub static EXPECTED_ANSWER: &'static str = "134043";

pub fn solve() -> ~str {
    let len = 4;
    let num_factor = 4;

    let mut cnt = 0;
    for n in iterator::count(1u, 1) {
        if prime::factorize(n).len_() != num_factor {
            cnt = 0;
            loop;
        }

        cnt += 1;
        if cnt == len {
            return (n + 1 - len).to_str();
        }
    }

    util::unreachable();
}
