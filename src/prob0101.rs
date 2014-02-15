#[crate_id = "prob0101"];
#[crate_type = "rlib"];

extern mod num;
extern mod math;

use std::vec;
use std::num::{Zero, One};
use num::bigint::BigInt;
use num::rational::Ratio;
use math::poly;

pub static EXPECTED_ANSWER: &'static str = "37076114526";

fn u(n: BigInt) -> BigInt {
    let mut sum: BigInt = Zero::zero();
    let mut prod = One::one();
    for _ in range(0, 11) {
        sum = sum + prod;
        prod = prod * (-n);
    }
    sum
}

// Lagrange Interpolating with Naville's algorithm
fn op(ns: &[(BigInt, BigInt)]) -> ~[Ratio<BigInt>] {
    let mut poly = ~[];
    for i in range(0, ns.len()) {
        let (ref xi, ref yi) = ns[i];
        let mut term = ~[ Ratio::from_integer(yi.clone()) ];
        for j in range(0, ns.len()) {
            if i == j { continue }

            let (ref xj, ref _yj) = ns[j];
            term = poly::mul(term, [Ratio::new(-xj, xi - *xj), Ratio::new(One::one(), xi - *xj)]);
        }
        poly = poly::add(poly, term);
    }
    poly
}

// y = y1 (x - x2)(x - x3) / (x1 - x2)(x1 - x3)
//   + y2 (x - x1)(x - x3) / (x2 - x1)(x2 - x3)
//   + y3 (x - x1)(x - x2) / (x3 - x1)(x3 - x2)
pub fn solve() -> ~str {
    let un = vec::from_fn(11, |n| (FromPrimitive::from_uint(n + 1).unwrap(),
                                   u(FromPrimitive::from_uint(n + 1).unwrap())));
    let mut sum: BigInt = Zero::zero();
    for i in range(1, un.len()) {
        let poly = op(un.slice(0, i)).move_iter().map(|x| x.numer().clone()).to_owned_vec();
        sum = sum + poly::eval(poly, FromPrimitive::from_uint(i + 1).unwrap());
    }
    sum.to_str()
}
