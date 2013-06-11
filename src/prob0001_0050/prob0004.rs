#[link(name = "prob0004", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::{str, uint};
use std::iterator::{IteratorUtil};
use common::problem::{Problem};

pub static problem: Problem<'static> = Problem {
    id: 4,
    answer: "906609",
    solver: solve
};

fn to_palindromic(n: uint, dup_flag: bool) -> uint {
    let ns = n.to_str();
    let mut rv = ns.rev_iter();
    if dup_flag { rv.next(); }

    let chars: ~[char] = ns.iter().chain(rv).collect();
    return uint::from_str(str::from_chars(chars)).get();
}

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
    for [false, true].each |&dup_flag| {
        for uint::range_rev(999, 99) |seed| {
            let num = to_palindromic(seed, dup_flag);
            for dividable_pairs(num, 100, 999) |d1, d2| {
                return (d1 * d2).to_str();
            }
        }
    }

    fail!();
}

