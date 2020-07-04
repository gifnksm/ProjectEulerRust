//! [Problem 38](https://projecteuler.net/problem=38) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use integer::Integer;
use iter::Permutations;
use std::cmp::Reverse;

fn compute() -> String {
    let nums = &[9, 8, 7, 6, 5, 4, 3, 2, 1];
    let radix = 10;

    let mut ans = 0;
    for (ds, rest) in Permutations::new(nums, 4) {
        let n: u32 = Integer::from_digits(ds.iter().rev().copied(), radix);
        let mut ds2 = (n * 2).into_digits(radix).collect::<Vec<_>>();
        ds2.sort_by_key(|&k| Reverse(k));

        if ds2 == rest {
            ans = n;
            break;
        }
    }
    format!("{}{}", ans, ans * 2)
}

fn solve() -> String {
    compute()
}

common::problem!("932718654", solve);
