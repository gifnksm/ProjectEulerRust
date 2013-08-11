#[link(name = "prob0101", vers = "0.0")];
#[crate_type = "lib"];

extern mod extra;
extern mod common;

use std::vec;
use std::num::{Zero, One};
use extra::bigint::BigInt;
use extra::rational::Ratio;
use common::poly;

pub static EXPECTED_ANSWER: &'static str = "37076114526";

fn u(n: BigInt) -> BigInt {
    let mut sum = Zero::zero::<BigInt>();
    let mut prod = One::one();
    do 11.times {
        sum = sum + prod;
        prod = prod * (-n);
    }
    return sum;
}

// Lagrange Interpolating with Naville's algorithm
fn op(ns: &[(BigInt, BigInt)]) -> ~[Ratio<BigInt>] {
    let mut poly = ~[];
    for i in range(0, ns.len()) {
        let (ref xi, ref yi) = ns[i];
        let mut term = ~[ Ratio::from_integer(yi.clone()) ];
        for j in range(0, ns.len()) {
            if i == j { loop; }

            let (ref xj, ref _yj) = ns[j];
            term = poly::mul(term, [Ratio::new(-xj, xi - *xj), Ratio::new(One::one(), xi - *xj)]);
        }
        poly = poly::add(poly, term);
    }
    return poly;
}

// y = y1 (x - x2)(x - x3) / (x1 - x2)(x1 - x3)
//   + y2 (x - x1)(x - x3) / (x2 - x1)(x2 - x3)
//   + y3 (x - x1)(x - x2) / (x3 - x1)(x3 - x2)
pub fn solve() -> ~str {
    let un = vec::from_fn(11, |n| (BigInt::from_uint(n + 1), u(BigInt::from_uint(n + 1))));
    let mut sum = Zero::zero::<BigInt>();
    for i in range(1, un.len()) {
        let poly = op(un.slice(0, i)).move_iter().map(|x| x.numer).to_owned_vec();
        sum = sum + poly::eval(poly, BigInt::from_uint(i + 1));
    }
    return sum.to_str();
}
