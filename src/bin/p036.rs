//! [Problem 36](https://projecteuler.net/problem=36) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#[macro_use(problem)] extern crate common;
extern crate integer;

use integer::Integer;

fn compute(limit: uint) -> uint {
    let order_array = &[ 1u, 10, 100, 1000, 1000, 10000 ];
    let mut sum = 0;
    for i in range(0, order_array.len() - 1) {
        let tf = [true, false];
        for &duplicate in tf.iter() {
            let (start, end) = (order_array[i], order_array[i + 1]);
            for n in range(start, end) {
                let n = n.into_palindromic(10, duplicate);
                if n >= limit { break; }
                if n.is_palindromic(2) {
                    sum += n;
                }
            }
        }
    }

    sum
}

fn solve() -> String {
    compute(1000000).to_string()
}

problem!("872187", solve);
