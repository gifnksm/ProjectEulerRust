#[link(name = "prob0004", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::uint;
use common::calc;
use common::extiter::Range;

pub static expected_answer: &'static str = "906609";

fn dividable_pairs(num: uint, min: uint, max: uint, f: &fn(uint, uint) -> bool) -> bool {
    let mut div = uint::max(uint::div_ceil(num, max), min);
    while div * div <= num && div <= max {
        if num % div == 0 {
            if !f(div, num / div) { return false; }
        }
        div += 1;
    }
    return true;
}

pub fn solve() -> ~str {
    let it1 = Range::new_rev(999u, 99).transform(|seed| {
        calc::to_palindromic(seed, 10, false)
    });
    let it2 = Range::new_rev(999u, 99).transform(|seed| {
        calc::to_palindromic(seed, 10, true)
    });

    let mut it = it1.chain_(it2);
    for it.advance |num| {
        for dividable_pairs(num, 100, 999) |d1, d2| {
            return (d1 * d2).to_str();
        }
    }

    fail!();
}

