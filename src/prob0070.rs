#[link(name = "prob0070", vers = "0.0")];
#[crate_type = "lib"];

extern mod math;

use std::float;
use math::{numconv, prime};

pub static EXPECTED_ANSWER: &'static str = "8319823";

pub fn solve() -> ~str {
    let limit = 10000000;

    // n / phi(n) = 1 / \Pi_{k=1}^d (1 - 1/p_k)
    // => p^k / phi(p^k) = p / phi(p)
    // p is greater then n / phi(p) is less
    //
    // phi(p) = p - 1 (if p is prime) => phi(p) is not permutation of p
    // phi(p1 * p2) = (p1 - 1) * (p2 - 1)

    let mut min_n   = 0;
    let mut min_n_phi = float::infinity;
    for p1 in prime::iter() {
        if p1 * p1 > limit { break; }
        for p2 in prime::iter() {
            if p2 < p1 { loop; }

            let n = p1 * p2;
            if n > limit { break; }

            let phi = (p1 - 1) * (p2 - 1);
            let ds1 = numconv::to_digit_histogram(n);
            let ds2 = numconv::to_digit_histogram(phi);
            if ds1 != ds2 { loop; }

            let n_phi = (n as float) / (phi as float);
            if n_phi < min_n_phi {
                min_n_phi = n_phi;
                min_n     = n;
            }
        }
    }

    return min_n.to_str();
}
