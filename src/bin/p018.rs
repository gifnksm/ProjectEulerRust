//! [Problem 18](https://projecteuler.net/problem=18) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use std::{cmp, num::ParseIntError};

const TRIANGLE: &str = "
75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23
";

fn parse(s: &str) -> Result<Vec<Vec<u32>>, ParseIntError> {
    s.lines()
        .filter(|l| !l.is_empty())
        .map(|line| line.split_ascii_whitespace().map(|s| s.parse()).collect())
        .collect()
}

fn compute(input: &[Vec<u32>]) -> u32 {
    let len = input.len();
    let init = &input[..len - 1];
    let last = input.last().unwrap();
    init.iter().rev().fold(last.to_vec(), |mut total, elm| {
        for (i, &e) in elm.iter().enumerate() {
            total[i] = e + cmp::max(total[i], total[i + 1]);
        }
        total
    })[0]
}

fn solve() -> String {
    compute(&parse(TRIANGLE).unwrap()).to_string()
}

common::problem!("1074", solve);

#[cfg(test)]
mod tests {
    const TRIANGLE: &str = "
    3
    7 4
    2 4 6
    8 5 9 3";

    #[test]
    fn small() {
        assert_eq!(23, super::compute(&super::parse(TRIANGLE).unwrap()));
    }
}
