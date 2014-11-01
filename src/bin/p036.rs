#![warn(unused, bad_style,
        unused_qualifications, unused_typecasts, unused_results)]

extern crate common;
extern crate integer;

use common::Solver;
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

fn main() { Solver::new("872187", solve).run(); }
