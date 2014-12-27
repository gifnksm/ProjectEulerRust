//! [Problem 93](https://projecteuler.net/problem=93) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(phase, slicing_syntax)]

#[phase(plugin, link)] extern crate common;
extern crate "iter" as itercrate;
extern crate num;

use std::iter;
use itercrate::{BitCombination, CombinationOverlap};
use num::{Signed, Zero};
use num::rational::Ratio;

#[deriving(Copy, Clone, Eq, PartialEq)]
enum Op { Add, Sub, Mul, Div }

struct Nums {
    comb: BitCombination
}

impl Nums {
    fn new() -> Nums {
        Nums { comb: BitCombination::new(4, 9) }
    }
}

impl Iterator<[uint, .. 4]> for Nums {
    fn next(&mut self) -> Option<[uint, .. 4]> {
        self.comb.next().map(|bits| {
            let mut result = [0, .. 4];
            for (i, n) in bits.iter().enumerate() {
                result[i] = n + 1;
            }
            result
        })
    }
}

fn apply(a: Ratio<int>, b: Ratio<int>, op: Op, f: &mut |Ratio<int>|) {
    match op {
        Op::Add => { (*f)(a + b) }
        Op::Mul => { (*f)(a * b) }
        Op::Sub => { (*f)(a - b); (*f)(b - a) }
        Op::Div => {
            if !b.is_zero() { (*f)(a / b) }
            if !a.is_zero() { (*f)(b / a) }
        }
    }
}

fn evaluate(num: &[uint], op: &[Op], f: &mut |Ratio<int>|) {
    // 4ops:
    //   n op 3ops
    //   3ops op n (if op = Sub/Div)
    //   2ops op 2ops
    //   2ops op 2ops (if op = Sub/Div)
    // 3ops:
    //   n op 2ops
    //   2op op n
    // 2ops:
    //   n op n
    assert_eq!(num.len() - 1, op.len());
    match num.len() {
        1 => { (*f)(Ratio::from_integer(num[0] as int)) }
        2 => {
            let a = Ratio::from_integer(num[0] as int);
            let b = Ratio::from_integer(num[1] as int);
            apply(a, b, op[0], f);
        }
        3 => {
            let a = Ratio::from_integer(num[0] as int);
            evaluate(&[num[1], num[2]], op[1 ..], &mut |b| apply(a, b, op[0], f));

            let a = Ratio::from_integer(num[1] as int);
            evaluate(&[num[2], num[0]], op[1 ..], &mut |b| apply(a, b, op[0], f));

            let a = Ratio::from_integer(num[2] as int);
            evaluate(&[num[0], num[1]], op[1 ..], &mut |b| apply(a, b, op[0], f));
        }
        4 => {
            let a = Ratio::from_integer(num[0] as int);
            evaluate(&[num[1], num[2], num[3]], op[1 ..], &mut |b| apply(a, b, op[0], f));

            let a = Ratio::from_integer(num[1] as int);
            evaluate(&[num[0], num[2], num[3]], op[1 ..], &mut |b| apply(a, b, op[0], f));

            let a = Ratio::from_integer(num[2] as int);
            evaluate(&[num[0], num[1], num[3]], op[1 ..], &mut |b| apply(a, b, op[0], f));

            let a = Ratio::from_integer(num[3] as int);
            evaluate(&[num[0], num[1], num[2]], op[1 ..], &mut |b| apply(a, b, op[0], f));

            evaluate(&[num[0], num[1]], &[op[1]], &mut |a| {
                evaluate(&[num[2], num[3]], &[op[2]], &mut |b| apply(a, b, op[0], f))
            });
            evaluate(&[num[0], num[2]], &[op[1]], &mut |a| {
                evaluate(&[num[1], num[3]], &[op[2]], &mut |b| apply(a, b, op[0], f))
            });
            evaluate(&[num[0], num[3]], &[op[1]], &mut |a| {
                evaluate(&[num[1], num[2]], &[op[2]], &mut |b| apply(a, b, op[0], f))
            });

            if op[1] != op[2] {
                evaluate(&[num[1], num[2]], &[op[1]], &mut |a| {
                    evaluate(&[num[0], num[3]], &[op[2]], &mut |b| apply(a, b, op[0], f))
                });
                evaluate(&[num[1], num[3]], &[op[1]], &mut |a| {
                    evaluate(&[num[0], num[2]], &[op[2]], &mut |b| apply(a, b, op[0], f))
                });
                evaluate(&[num[2], num[3]], &[op[1]], &mut |a| {
                    evaluate(&[num[0], num[1]], &[op[2]], &mut |b| apply(a, b, op[0], f))
                });
            }
        }
        _ => unreachable!()
    }
}

fn count_seqlen(num_set: &[uint, .. 4]) -> uint {
    let mut set = [false, .. 3025];

    for op_set in CombinationOverlap::new(&[Op::Add, Op::Sub, Op::Mul, Op::Div], num_set.len() - 1) {
        for ops in op_set.permutations() {
            evaluate(num_set[], ops[], &mut |n| {
                if n.is_integer() && n.numer().is_positive() {
                    set[n.to_integer() as uint] = true;
                }
            })
        }
    }

    iter::count(1, 1)
        .take_while(|&i| set[i])
        .last()
        .unwrap_or(0)
}

fn solve() -> String {
    let seq = Nums::new().max_by(count_seqlen).unwrap();
    format!("{}{}{}{}", seq[0], seq[1], seq[2], seq[3])
}

problem!("1258", solve);

#[cfg(test)]
mod tests {
    use super::Nums;

    #[test]
    fn nums() {
        let mut nums = Nums::new();
        for a in range(1, 10) {
            for b in range(a + 1, 10) {
                for c in range(b + 1, 10) {
                    for d in range(c + 1, 10) {
                        assert_eq!(Some([a, b, c, d]), nums.next());
                    }
                }
            }
        }
        assert_eq!(None, nums.next());
    }

    #[test]
    fn count_seqlen() {
        assert_eq!(28, super::count_seqlen(&[1, 2, 3, 4]));
    }
}
