//! [Problem 67](https://projecteuler.net/problem=67) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature( str_words)]

#[macro_use(problem)] extern crate common;

use std::cmp;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;

fn solve(file: File) -> io::Result<String> {
    let mut triangle = BufReader::new(file).lines()
        .filter_map(|line| line.ok())
        .filter(|line| !line.is_empty())
        .map(|line| line.words().filter_map(|s| StrExt::parse(s).ok()).collect::<Vec<u32>>())
        .collect::<Vec<_>>();

    let last = triangle.pop().unwrap();    let ans = triangle.iter()
        .rev()
        .fold(last, |prev, elem| {
            (0 .. elem.len()).map(|i| elem[i] + cmp::max(prev[i], prev[i + 1])).collect()
        })[0];

    Ok(ans.to_string())
}

problem!("7273", "p067_triangle.txt", solve);

