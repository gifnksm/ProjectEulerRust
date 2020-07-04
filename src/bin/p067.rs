//! [Problem 67](https://projecteuler.net/problem=67) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use std::{
    cmp,
    fs::File,
    io::{self, prelude::*, BufReader},
};

fn solve(file: File) -> io::Result<String> {
    let mut triangle = BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<_>>();

    let last = triangle.pop().unwrap();
    let ans = triangle.iter().rev().fold(last, |prev, elem| {
        (0..elem.len())
            .map(|i| elem[i] + cmp::max(prev[i], prev[i + 1]))
            .collect()
    })[0];

    Ok(ans.to_string())
}

common::problem!("7273", "p067_triangle.txt", solve);
