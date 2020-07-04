//! [Problem 72](https://projecteuler.net/problem=72) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use prime::PrimeSet;

fn compute(limit: u64) -> u64 {
    let prime = PrimeSet::new();

    let mut v = (0..limit + 1).collect::<Vec<_>>();
    v[1] = 0;

    for p in &prime {
        if p > limit {
            break;
        }
        for n in (p..limit + 1).step_by(p as usize) {
            v[n as usize] = (v[n as usize] * (p - 1)) / p;
        }
    }

    v.into_iter().sum()
}

fn solve() -> String {
    compute(1000000).to_string()
}

common::problem!("303963552391", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn eight() {
        assert_eq!(21, super::compute(8));
    }
}
