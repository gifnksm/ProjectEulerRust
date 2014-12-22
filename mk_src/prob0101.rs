#![crate_name = "prob0101"]
#![crate_type = "rlib"]

extern crate num;
extern crate math;

use num::{Zero, One, BigInt};
use num::rational::Ratio;
use math::poly::Poly;

pub const EXPECTED_ANSWER: &'static str = "37076114526";

fn u(n: BigInt) -> BigInt {
    let mut sum: BigInt = Zero::zero();
    let mut prod = One::one();
    for _ in range(0u, 11) {
        sum = sum + &prod;
        prod = &prod * (-&n);
    }
    sum
}

// Lagrange Interpolating with Naville's algorithm
fn op(ns: &[(BigInt, BigInt)]) -> Poly<Ratio<BigInt>> {
    let mut poly = Poly::new(vec![]);
    for i in range(0, ns.len()) {
        let (ref xi, ref yi) = ns[i];
        let mut term = Poly::new(vec![ Ratio::from_integer(yi.clone()) ]);
        for j in range(0, ns.len()) {
            if i == j { continue }

            let (ref xj, ref _yj) = ns[j];
            term = term * Poly::new(vec![Ratio::new(-xj, xi - xj),
                                         Ratio::new(One::one(), xi - xj)]);
        }
        poly = poly + term;
    }
    poly
}

// y = y1 (x - x2)(x - x3) / (x1 - x2)(x1 - x3)
//   + y2 (x - x1)(x - x3) / (x2 - x1)(x2 - x3)
//   + y3 (x - x1)(x - x2) / (x3 - x1)(x3 - x2)
pub fn solve() -> String {
    let un = Vec::from_fn(11, |n| (FromPrimitive::from_uint(n + 1).unwrap(),
                                   u(FromPrimitive::from_uint(n + 1).unwrap())));
    let mut sum: BigInt = Zero::zero();
    for i in range(1, un.len()) {
        let poly = Poly::new(op(un.slice(0, i))
                             .as_slice()
                             .iter()
                             .map(|x| x.numer().clone())
                             .collect());
        sum = sum + poly.eval(FromPrimitive::from_uint(i + 1).unwrap());
    }
    sum.to_string()
}
