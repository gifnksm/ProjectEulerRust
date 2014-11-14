#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

extern crate num;
extern crate common;
extern crate integer;

use num::Integer as NumInteger;
use common::Solver;
use integer::Integer;

fn compute(min: uint, max: uint) -> uint {
    let r = range(min, max + 1);
    let it1 = r.rev().map(|seed| seed.into_palindromic(10, true));
    let it2 = r.rev().map(|seed| seed.into_palindromic(10, false));

    for p in it1.chain(it2) {
        for n in r.clone() {
            if n * n > p { break; }

            let (d, r) = p.div_rem(&n);
            if r == 0 && min <= d && d <= max {
                return p
            }
        }
    }

    unreachable!()
}

fn solve() -> String { compute(100, 999).to_string() }

fn main() { Solver::new("906609", solve).run(); }

#[cfg(test)]
mod tests {
    #[test]
    fn two_digits_palindromic() {
        assert_eq!(9009, super::compute(10, 99));
    }
}
