#[link(name = "prob0036", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::uint;
use common::calc;

pub static expected_answer: &'static str = "872187";

pub fn solve() -> ~str {
    let order_array = &[ 1, 10, 100, 1000, 1000, 10000 ];
    let mut sum = 0;
    for uint::range(0, order_array.len() - 1) |i| {
        let tf = [true, false];
        for tf.iter().advance |b| {
            let (start, end) = (order_array[i], order_array[i + 1]);
            for uint::range(start, end) |n| {
                let n = calc::to_palindromic(n, 10, *b);
                if n >= 1000000 { break; }
                if calc::is_palindromic(n, 2) {
                    sum += n;
                }
            }
        }
    }

    return sum.to_str();
}

