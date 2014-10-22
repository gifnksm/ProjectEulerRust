#![warn(unused, bad_style,
        unnecessary_qualification, unnecessary_typecast, unused_result)]

#![feature(slicing_syntax)]

extern crate common;
extern crate integer;
extern crate iter;

use std::collections::HashSet;
use std::iter::AdditiveIterator;
use common::Solver;
use integer::Integer;
use iter::Permutation;

// possible num of digits combinations
// 1 x 1 = 7 : NG 10 * 10
// 1 x 2 = 6 : NG 10 * 100
// 1 x 3 = 5 : NG 10 * 1000 = 10000
// 1 x 4 = 4 : OK
// 2 x 2 = 5 : NG 100 * 100 = 10000
// 2 x 3 = 4 : OK
// 3 x 3 = 3 : NG

fn compute() -> uint {
    let radix = 10;
    let digits = &[1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut products = HashSet::new();

    // 1 x 4 = 4
    // a b = c
    // 1 < a < 10
    // 1000 < b < 10000
    // 1000 < c = ab < 10000 => 1000 / a < b < 10000 / a
    //  => 1000 < b < 10000 / a
    for (p1, r1) in Permutation::new(digits, 1) {
        for (p2, r2) in Permutation::new(r1[], 4) {
            let a = Integer::from_digits(p1.iter().map(|&x| x), radix);
            let b = Integer::from_digits(p2.iter().map(|&x| x), radix);
            let c: uint = a * b;
            let mut c_digits = c.into_digits(radix).collect::<Vec<_>>();
            c_digits.sort();
            if r2 == c_digits { products.insert(c); }
        }
    }

    // 2 x 3 = 4
    // a b = c
    // 10   < a < 100
    // 100  < b < 1000
    // 1000 < c = ab < 10000 => 1000 / a < b < 10000 / a
    // => 100 < b < 10000 / a
    for (p1, r1) in Permutation::new(digits, 2) {
        for (p2, r2) in Permutation::new(r1[], 3) {
            let a = Integer::from_digits(p1.iter().map(|&x| x), radix);
            let b = Integer::from_digits(p2.iter().map(|&x| x), radix);
            let c: uint = a * b;
            let mut c_digits = c.into_digits(radix).collect::<Vec<_>>();
            c_digits.sort();
            if r2 == c_digits { products.insert(c); }
        }
    }

    products.into_iter().sum()
}

fn solve() -> String {
    compute().to_string()
}

fn main() { Solver::new("45228", solve).run(); }
