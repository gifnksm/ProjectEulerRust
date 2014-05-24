#![crate_id = "prob0070"]
#![crate_type = "rlib"]

extern crate math;

use std::f64;
use math::numconv;
use math::prime::Prime;

pub static EXPECTED_ANSWER: &'static str = "8319823";

pub fn solve() -> StrBuf {
    let limit = 10000000;

    // n / phi(n) = 1 / \Pi_{k=1}^d (1 - 1/p_k)
    // => p^k / phi(p^k) = p / phi(p)
    // p is greater then n / phi(p) is less
    //
    // phi(p) = p - 1 (if p is prime) => phi(p) is not permutation of p
    // phi(p1 * p2) = (p1 - 1) * (p2 - 1)

    let prime = Prime::new();
    let mut min_n   = 0;
    let mut min_n_phi = f64::INFINITY;
    for p1 in prime.iter() {
        if p1 * p1 > limit { break }
        for p2 in prime.iter() {
            if p2 < p1 { continue }

            let n = p1 * p2;
            if n > limit { break }

            let phi = (p1 - 1) * (p2 - 1);
            let ds1 = numconv::to_digit_histogram(n);
            let ds2 = numconv::to_digit_histogram(phi);
            if ds1 != ds2 { continue }

            let n_phi = (n as f64) / (phi as f64);
            if n_phi < min_n_phi {
                min_n_phi = n_phi;
                min_n     = n;
            }
        }
    }

    min_n.to_str()
}
