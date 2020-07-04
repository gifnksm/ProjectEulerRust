//! [Problem 52](https://projecteuler.net/problem=52) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use integer::Integer;

fn compute() -> u32 {
    let radix = 10;
    let repeat = 6;
    let mut n = 0;
    let mut order = 1;
    let mut limit = (order - 1) / repeat;

    loop {
        n += 1;
        if n > limit {
            // skip if the num of digits of n * repeat is not the same with n.
            n = order;
            order *= radix;
            limit = (order - 1) / repeat;
        }

        let ds = n.into_digit_histogram();

        // n * 5 must contains 0 or 5.
        if ds[0] == 0 && ds[5] == 0 {
            continue;
        }

        // n * 2, n * 4 must contains some evens.
        if ds[0] == 0 && ds[2] == 0 && ds[4] == 0 && ds[6] == 0 && ds[8] == 0 {
            continue;
        }

        if ds != (n * 2).into_digit_histogram() {
            continue;
        }
        if ds != (n * 3).into_digit_histogram() {
            continue;
        }
        if ds != (n * 4).into_digit_histogram() {
            continue;
        }
        if ds != (n * 5).into_digit_histogram() {
            continue;
        }
        if ds != (n * 6).into_digit_histogram() {
            continue;
        }

        return n;
    }
}

fn solve() -> String {
    compute().to_string()
}

common::problem!("142857", solve);
