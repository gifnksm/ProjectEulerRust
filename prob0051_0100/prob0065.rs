use std::bigint::{ BigUint };

use core::util::{ swap };

use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 65,
    answer: "272",
    solver: solve
};

fn napier_seq(i: uint) -> uint {
    match i {
        0 => 2,
        i if i % 3 == 2 => 2 * (i + 1) / 3,
        _ => 1
    }
}

fn fold_cont_frac(ns: &[uint]) -> (BigUint, BigUint) {
    let mut numer = BigUint::from_uint(1);
    let mut denom = BigUint::from_uint(0);

    for ns.each_reverse |&n| {
        swap(&mut numer, &mut denom);
        numer = numer + BigUint::from_uint(n) * denom;
    }

    return (numer, denom);
}

fn solve() -> ~str {
    let len = 100;

    let napier = vec::from_fn(len, napier_seq);

    let (n, _d) = fold_cont_frac(napier);
    let mut sum = 0;
    for str::each_char(n.to_str()) |c| {
        sum += char::to_digit(c, 10).get();
    }
    return sum.to_str();
}

