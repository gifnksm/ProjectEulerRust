//! [Problem 44](https://projecteuler.net/problem=44) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

fn nth_pentagonal(i: u32) -> u32 {
    let n = i + 1;
    n * (3 * n - 1) / 2
}

fn is_pentagonal(n: u32, table: &[u32]) -> bool {
    if *table.last().unwrap() < n {
        panic!()
    }
    table.binary_search(&n).is_ok()
}

// P[k] + P[j] = P[m]
// P[k] - P[j] = P[n]
//
// 2*P[k] = P[m] + P[n] > 0
// 2*P[j] = P[m] - P[n] > 0
//
// find minimum n, where n < m
fn solve() -> String {
    let limit = 10000;
    let pentagonals = (0..limit).map(nth_pentagonal).collect::<Vec<_>>();

    for m in 0.. {
        let pm = pentagonals[m];
        for n in 0..m {
            let pn = pentagonals[n];
            if (pm - pn) % 2 != 0 {
                continue;
            }
            if is_pentagonal(pm - pn, &pentagonals) && is_pentagonal(pm + pn, &pentagonals) {
                return (pm - pn).to_string();
            }
        }
    }
    unreachable!()
}

common::problem!("5482660", solve);
