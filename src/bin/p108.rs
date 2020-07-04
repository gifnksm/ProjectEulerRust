//! [Problem 108](https://projecteuler.net/problem=108) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use prime::{Factorize, PrimeSet};

// 1/x + 1/y = 1/n
// a := x - n >= 0
// b := y - n >= 0
//
// 1/(n+a) + 1/(n+b) = 1/n
// n(n+b) + n(n+a) = (n+a)(n+b)
// 2n^2 + n(a+b) = n^2 + n(a+b) + ab
// n^2 = ab

fn num_pairs(ps: &PrimeSet, n: u64) -> u64 {
    let prod = n
        .factorize(ps)
        .map(|(_base, exp)| 2 * (exp as u64) + 1)
        .product::<u64>();
    (prod - 1) / 2 + 1
}

fn solve() -> String {
    let n = 1000;
    let ps = PrimeSet::new();
    (1..).find(|&i| num_pairs(&ps, i) > n).unwrap().to_string()
}

common::problem!("180180", solve);

#[cfg(test)]
mod tests {
    use prime::PrimeSet;

    #[test]
    fn test_num_pairs() {
        let ps = PrimeSet::new();
        assert_eq!(super::num_pairs(&ps, 4), 3);
        assert_eq!(super::num_pairs(&ps, 1260), 113);
    }
}
