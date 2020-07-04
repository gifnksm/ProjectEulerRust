//! [Problem 58](https://projecteuler.net/problem=58) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use prime::PrimeSet;

fn compute(numer: u32, denom: u32) -> u64 {
    let ps = PrimeSet::new();
    let mut side = 1;
    let mut num_prime = 0;
    let mut num_total = 1;

    loop {
        side += 2;
        let rb = side * side;
        let lb = rb - side + 1;
        let lt = lb - side + 1;
        let rt = lt - side + 1;
        if ps.contains(lb) {
            num_prime += 1;
        }
        if ps.contains(lt) {
            num_prime += 1;
        }
        if ps.contains(rt) {
            num_prime += 1;
        }
        num_total += 4;
        if num_prime * denom < numer * num_total {
            return side;
        }
    }
}

fn solve() -> String {
    compute(1, 10).to_string()
}

common::problem!("26241", solve);
