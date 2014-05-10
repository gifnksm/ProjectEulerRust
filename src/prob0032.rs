#![crate_id = "prob0032"]
#![crate_id = "prob0032"]
#![crate_type = "rlib"]
#![crate_type = "rlib"]

extern crate collections;
extern crate common;
extern crate math;

use std::iter::AdditiveIterator;
use collections::HashSet;
use common::calc;
use math::numconv;

pub static EXPECTED_ANSWER: &'static str = "45228";

// possible num of digits combinations
// 1 x 1 = 7 : NG 10 * 10
// 1 x 2 = 6 : NG 10 * 100
// 1 x 3 = 5 : NG 10 * 1000 = 10000
// 1 x 4 = 4 : OK
// 2 x 2 = 5 : NG 100 * 100 = 10000
// 2 x 3 = 4 : OK
// 3 x 3 = 3 : NG

pub fn solve() -> ~str {
    let digits = &[1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut answer = HashSet::new();

    // 1 x 4 = 4
    // a b = c
    // 1 < a < 10
    // 1000 < b < 10000
    // 1000 < c = ab < 10000 => 1000 / a < b < 10000 / a
    //  => 1000 < b < 10000 / a
    calc::permutate_num(digits, 1, 0, 9, |a, ds| {
            calc::permutate_num(ds, 4, 1000, 9999 / a, |b, ds| {
                    let c = a * b;
                    let mut c_digits = numconv::to_digits(c, 10).collect::<Vec<uint>>();
                    c_digits.sort_by(|a, b| a.cmp(b));
                    if ds == c_digits.as_slice() { answer.insert(c); }
                    true
                });
            true
        });

    // 2 x 3 = 4
    // a b = c
    // 10   < a < 100
    // 100  < b < 1000
    // 1000 < c = ab < 10000 => 1000 / a < b < 10000 / a
    // => 100 < b < 10000 / a
    calc::permutate_num(digits, 2, 10, 99, |a, ds| {
            calc::permutate_num(ds, 3, 100, 9999 / a, |b, ds| {
                    let c = a * b;
                    let mut c_digits = numconv::to_digits(c, 10).collect::<Vec<uint>>();
                    c_digits.sort_by(|a, b| a.cmp(b));
                    if ds == c_digits.as_slice() { answer.insert(c); }
                    true
                });
            true
        });

    return answer.move_iter().sum().to_str();
}
