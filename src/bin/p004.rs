//! [Problem 4](https://projecteuler.net/problem=4) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use integer::Integer;
use num_integer::Integer as NumInteger;

fn compute(min: u32, max: u32) -> u32 {
    let r = min..(max + 1);
    let it1 = r.clone().rev().map(|seed| seed.into_palindromic(10, true));
    let it2 = r.rev().map(|seed| seed.into_palindromic(10, false));

    for p in it1.chain(it2) {
        for n in min..(max + 1) {
            if n * n > p {
                break;
            }

            let (d, r) = p.div_rem(&n);
            if r == 0 && min <= d && d <= max {
                return p;
            }
        }
    }

    unreachable!()
}

fn solve() -> String {
    compute(100, 999).to_string()
}

common::problem!("906609", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn two_digits_palindromic() {
        assert_eq!(9009, super::compute(10, 99));
    }
}
