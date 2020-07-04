//! [Problem 86](https://projecteuler.net/problem=86) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use seq::PrimitivePythagoreans;
use std::cmp::{self, Ordering};

fn get_count(m: u32) -> u32 {
    let mut cnt = 0;
    for max_a in 0..m {
        for (p, q, _) in PrimitivePythagoreans::new(max_a) {
            for k in 1..(m / q + 1) {
                cnt += k * p / 2;
            }

            for k in 1..(m / p + 1) {
                let end = cmp::min(k * p, k * q / 2) + 1;
                let start = k * q - k * p;
                if end > start {
                    cnt += end - start;
                }
            }
        }
    }
    cnt
}

// cuboid: (a, b, c),  a <= b <= c <= M
// => S = sqrt(c^2 + (a + b)^2)
fn get_min_m(limit: u32) -> u32 {
    let mut lim = 1;
    let mut cnt = get_count(lim);
    while cnt < limit {
        lim *= 2;
        cnt = get_count(lim);
    }

    let mut m = 0;
    while lim != 0 {
        let ix = m + (lim / 2);
        let cnt = get_count(ix);
        match cnt.cmp(&limit) {
            Ordering::Equal => break,
            Ordering::Less => {
                m = ix + 1;
                lim -= 1;
            }
            Ordering::Greater => {}
        }
        lim /= 2;
    }

    m
}

fn solve() -> String {
    get_min_m(1000000).to_string()
}

common::problem!("1818", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn exceed_two_thousand() {
        assert_eq!(100, super::get_min_m(2000));
    }

    #[test]
    fn get_count() {
        assert_eq!(1975, super::get_count(99));
        assert_eq!(2060, super::get_count(100));
    }
}
