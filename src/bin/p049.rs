//! [Problem 49](https://projecteuler.net/problem=49) solver.

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

fn solve() -> String {
    let ps = PrimeSet::new();
    let d = 3330;
    let (p1, p2, p3) = ps
        .iter()
        .skip_while(|&p| p < 1000)
        .take_while(|&p| p <= 9999 - 2 * d)
        .filter(|&p| p != 1487)
        .map(|p| (p, p + d, p + d + d))
        .filter(|&(_p1, p2, p3)| ps.contains(p3) && ps.contains(p2))
        .find(|&(p1, p2, p3)| {
            let hs1 = p1.into_digit_histogram();
            let hs2 = p2.into_digit_histogram();
            hs1 == hs2 && hs1 == p3.into_digit_histogram()
        })
        .unwrap();
    format!("{}{}{}", p1, p2, p3)
}

common::problem!("296962999629", solve);
