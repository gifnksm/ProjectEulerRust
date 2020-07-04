//! [Problem 70](https://projecteuler.net/problem=70) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use integer::Integer;
use prime::PrimeSet;
use std::f64;

fn compute(limit: u64) -> u64 {
    // n = \Pi_{k=1}^d p_k
    // n / phi(n) = 1 / \Pi_{k=1}^d (1 - 1/p_k)
    // => p^k / phi(p^k) = p / phi(p)
    // p is greater then n / phi(p) is less
    //
    // phi(p) = p - 1 (if p is prime) => phi(p) is not permutation of p
    // phi(p1 * p2) = (p1 - 1) * (p2 - 1)

    let ps = PrimeSet::new();
    let mut min_n = 0;
    let mut min_n_phi = f64::INFINITY;
    for p1 in &ps {
        if p1 * p1 > limit {
            break;
        }
        for p2 in &ps {
            if p2 < p1 {
                continue;
            }

            let n = p1 * p2;
            if n > limit {
                break;
            }

            let phi = (p1 - 1) * (p2 - 1);
            let ds1 = n.into_digit_histogram();
            let ds2 = phi.into_digit_histogram();
            if ds1 != ds2 {
                continue;
            }

            let n_phi = (n as f64) / (phi as f64);
            if n_phi < min_n_phi {
                min_n_phi = n_phi;
                min_n = n;
            }
        }
    }
    min_n
}

fn solve() -> String {
    compute(10000000).to_string()
}

common::problem!("8319823", solve);
