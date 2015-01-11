//! [Problem 126](https://projecteuler.net/problem=126) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#[macro_use(problem)] extern crate common;

use std::iter;

// cube size: (a, b, c)
// nth layer: 4(n-1)(n+a+b+c-2) + 2(ab+bc+ca)
fn f0(a: u32, b: u32, c: u32) -> u32 { 2 * (a*b + b*c + c*a) }

fn compute(sum: u32) -> u32 {
    let limit = sum * 20;
    let mut cnt = iter::repeat(0).take(limit as uint).collect::<Vec<_>>();

    for a in iter::count(1, 1) {
        if f0(a, 1, 1) > limit { break }

        for b in iter::range_inclusive(1, a) {
            if f0(a, b, 1) > limit { break }

            for c in iter::range_inclusive(1, b) {
                let p = f0(a, b, c);
                if p > limit { break; }
                let q = a + b + c - 2;

                for n in iter::count(1, 1) {
                    let f = 4*(n-1)*(n+q) + p;
                    if f as uint >= cnt.len() { break }
                    cnt[f as uint] += 1;
                }
            }
        }
    }

    cnt.iter()
        .position(|&n| n == sum)
        .unwrap() as u32
}

fn solve() -> String {
    compute(1000).to_string()
}

problem!("18522", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn ten() {
        assert_eq!(154, super::compute(10));
    }
}
