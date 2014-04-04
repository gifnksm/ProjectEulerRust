#![crate_id = "prob0093"]
#![crate_type = "rlib"]

extern crate num;
extern crate common;
extern crate math;

use std::iter;
use std::num::Zero;
use num::rational::{Rational, Ratio};
use common::calc;
use math::numconv;

pub static EXPECTED_ANSWER: &'static str = "1258";

#[deriving(Clone)]
enum Op { Add, Sub, Mul, Div }

#[inline(always)]
fn each_numseq(f: |&[Rational]|) {
    calc::combinate(range(1, 10).map(Ratio::from_integer).collect::<~[Ratio<int>]>(), 4, |x, _| {f(x); true});
}

#[inline(always)]
fn each_opseq(f: |&[Op]|) {
    let ops = ~[ Add, Sub, Mul, Div ];
    for i1 in range(0, ops.len()) {
        for i2 in range(0, ops.len()) {
            for i3 in  range(0, ops.len()) {
                f(&[ops[i1], ops[i2], ops[i3]]);
            }
        }
    }
}

fn each_value(num: &[Rational], op: &[Op], f: |Rational|) {
    assert_eq!(num.len() - 1, op.len());
    if num.len() == 1 { f(num[0]); return }

    calc::combinate(num, 1, |v1, rest| {
        let a = v1[0];
        each_value(rest, op.tailn(1), |b| {
            match op[0] {
                Add => { f(a + b) }
                Mul => { f(a * b) }
                Sub => { f(a - b); f(b - a) }
                Div => {
                    if !b.is_zero() { f(a / b) }
                    if !a.is_zero() { f(b / a) }
                }
            }
        });
        true
    });
}

fn count_seqlen(nums: &[Rational]) -> uint {
    let mut set = [false, .. 10000];

    each_opseq(|ops| {
        each_value(nums, ops, |n| {
            if n.is_integer() && n.numer().is_positive() {
                set[n.to_integer() as uint] = true;
            }
        });
    });

    iter::count(1u, 1)
        .take_while(|&i| set[i])
        .last()
        .unwrap_or(Zero::zero())
}

pub fn solve() -> ~str {
    let mut max_seq = ~"";
    let mut max_cnt = 0;
    each_numseq(|nums| {
        let cnt = count_seqlen(nums);
        if cnt > max_cnt {
            max_cnt = cnt;
            let ds = nums.rev_iter().map(|r| r.to_integer() as uint).collect::<~[uint]>();
            max_seq = numconv::from_digits(ds, 10).to_str();
        }
    });

    return max_seq;
}
