#[link(name = "prob0093", vers = "0.0")];
#[crate_type = "lib"];

extern mod extra;
extern mod common;

use std::iterator;
use std::num::Zero;
use std::hashmap::HashSet;
use extra::rational::{Rational, Ratio};
use common::calc;

pub static EXPECTED_ANSWER: &'static str = "1258";

enum Op { Add, Sub, Mul, Div }

#[inline(always)]
fn each_numseq(f: &fn(&[Rational]) -> bool) -> bool {
    for a in range(1, 10) {
        let ra = Ratio::from_integer(a);
        for b in range(a + 1, 10) {
            let rb = Ratio::from_integer(b);
            for c in range(b + 1, 10) {
                let rc = Ratio::from_integer(c);
                for d in range(c + 1, 10) {
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
    for i1 in range(0, ops.len()) {
        for i2 in range(0, ops.len()) {
            for i3 in  range(0, ops.len()) {
                if !f(&[ops[i1], ops[i2], ops[i3]]) { return false; }
            }
        }
    }
    return true;
}

fn each_value(num: &[Rational], op: &[Op], f: &fn(n: Rational) -> bool) -> bool {
    assert_eq!(num.len() - 1, op.len());
    if num.len() == 1 { return f(num[0]); }

    do calc::combinate(num, 1) |v1, rest| {
        let a = v1[0];
        do each_value(rest, op.tailn(1)) |b| {
            match op[0] {
                Add => { f(a + b) }
                Mul => { f(a * b) }
                Sub => {
                    f(a - b) && f(b - a)
                }
                Div => {
                    (b.is_zero() || f(a / b)) && (a.is_zero() || f(b / a))
                }
            }
        }
    }
}

fn count_seqlen(nums: &[Rational]) -> uint {
    let mut set = HashSet::new();

    do each_opseq |ops| {
        do each_value(nums, ops) |n| {
            if n.denom == 1 && n.numer > 0 {
                set.insert(n.numer as uint);
            }
            true
        };
        true
    };

    iterator::count(1u, 1)
        .take_while(|&i| set.contains(&i))
        .last_()
        .unwrap_or_default(0)
}

pub fn solve() -> ~str {
    let mut max_seq = ~"";
    let mut max_cnt = 0;
    do each_numseq |nums| {
        let cnt = count_seqlen(nums);
        if cnt > max_cnt {
            max_cnt = cnt;
            max_seq = calc::digits_to_num(nums.map(|r| r.numer as uint), 10).to_str();
        }
        true
    };

    return max_seq;
}
