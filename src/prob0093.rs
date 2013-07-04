#[link(name = "prob0093", vers = "0.0")];
#[crate_type = "lib"];

extern mod extra;
extern mod common;

use std::{uint, int, util};
use std::num::Zero;
use std::hashmap::HashSet;
use std::iterator::Counter;
use extra::rational::{Rational, Ratio};
use common::calc;

pub static EXPECTED_ANSWER: &'static str = "1258";

enum Op { Add, Sub, Mul, Div }

#[inline(always)]
fn each_numseq(f: &fn(&[Rational]) -> bool) -> bool {
    for int::range(1, 10) |a| {
        let ra = Ratio::from_integer(a);
        for int::range(a + 1, 10) |b| {
            let rb = Ratio::from_integer(b);
            for int::range(b + 1, 10) |c| {
                let rc = Ratio::from_integer(c);
                for int::range(c + 1, 10) |d| {
                    let rd = Ratio::from_integer(d);
                    if !f(&[ra, rb, rc, rd]) { return false; }
                }
            }
        }
    }
    return true;
}

#[inline(always)]
fn each_opseq(f: &fn(&[Op]) -> bool) -> bool {
    let ops = ~[ Add, Sub, Mul, Div ];
    for uint::range(0, ops.len()) |i1| {
        for uint::range(0, ops.len()) |i2| {
            for uint::range(0, ops.len()) |i3| {
                if !f(&[ops[i1], ops[i2], ops[i3]]) { return false; }
            }
        }
    }
    return true;
}

fn each_value(num: &[Rational], op: &[Op], f: &fn(n: Rational) -> bool) -> bool {
    assert_eq!(num.len() - 1, op.len());
    if num.len() == 1 { return f(num[0]); }

    for calc::combinate(num, 1) |v1, rest| {
        let a = v1[0];
        for each_value(rest, op.tailn(1)) |b| {
            match op[0] {
                Add => { if !f(a + b) { return false; } }
                Mul => { if !f(a * b) { return false; } }
                Sub => {
                    if !f(a - b) { return false; }
                    if !f(b - a) { return false; }
                }
                Div => {
                    if !b.is_zero() && !f(a / b) { return false; }
                    if !a.is_zero() && !f(b / a) { return false; }
                }
            }
        }
    }
    return true;
}

fn count_seqlen(nums: &[Rational]) -> uint {
    let mut set = HashSet::new();

    for each_opseq |ops| {
        for each_value(nums, ops) |n| {
            if n.denom == 1 && n.numer > 0 {
                set.insert(n.numer as uint);
            }
        }
    }

    let mut counter = Counter::new(1u, 1u);
    for counter.advance |i| {
        if !set.contains(&i) { return i - 1; }
    }

    util::unreachable();
}

pub fn solve() -> ~str {
    let mut max_seq = ~"";
    let mut max_cnt = 0;
    for each_numseq |nums| {
        let cnt = count_seqlen(nums);
        if cnt > max_cnt {
            max_cnt = cnt;
            max_seq = calc::digits_to_num(nums.map(|r| r.numer as uint), 10).to_str();
        }
    }

    return max_seq;
}
