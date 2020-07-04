//! [Problem 47](https://projecteuler.net/problem=47) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use prime::{Factorize, PrimeSet};

fn compute(len: usize, num_factor: usize) -> usize {
    let ps = PrimeSet::new();
    let mut cnt = 0;

    for n in 1.. {
        if n.factorize(&ps).count() != num_factor {
            cnt = 0;
            continue;
        }

        cnt += 1;
        if cnt == len {
            return n + 1 - len;
        }
    }

    unreachable!()
}

fn solve() -> String {
    compute(4, 4).to_string()
}

common::problem!("134043", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn two() {
        assert_eq!(14, super::compute(2, 2));
    }
    #[test]
    fn three() {
        assert_eq!(644, super::compute(3, 3));
    }
}
