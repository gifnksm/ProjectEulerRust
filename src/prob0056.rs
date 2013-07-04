#[link(name = "prob0056", vers = "0.0")];
#[crate_type = "lib"];

extern mod extra;


use std::{uint};
use std::num::{Zero};
use extra::bigint::{BigUint};

pub static EXPECTED_ANSWER: &'static str = "972";

fn digit_sum(n: uint) -> uint {
    let mut sum = 0;
    let mut d = n;
    while d > 0 {
        sum += d % 10;
        d = d / 10;
    }
    return sum;
}

pub fn solve() -> ~str {
    let ten = BigUint::from_uint(1000000000);

    let mut max = 0u;
    for uint::range(1, 100) |a| {
        let mut n = BigUint::from_uint(a);
        for 100.times {
            n = n * BigUint::from_uint(a);
            let mut sum = 0;
            let (d, m) = n.div_rem(&ten);
            let mut d = d;
            sum += digit_sum(m.to_uint());
            while !d.is_zero() {
                let (d0, m) = d.div_rem(&ten);
                sum += digit_sum(m.to_uint());
                d = d0;
            }
            // following code causes core dumps...
            // let s = n.to_str();
            // for str::each_char(s) |c| {
            //     let d = char::to_digit(c, 10);
            //     sum += d.get();
            // }
            max = uint::max(max, sum);
        }
    }

    return max.to_str();
}
