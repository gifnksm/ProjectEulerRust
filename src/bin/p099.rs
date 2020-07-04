//! [Problem 99](https://projecteuler.net/problem=99) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
};

fn solve(file: File) -> io::Result<String> {
    let mut max_val = 0.0;
    let mut max_idx = 0;

    for (line, idx) in BufReader::new(file).lines().zip(1..) {
        let line = line?;
        let line = line.trim();
        let i = line.find(',').unwrap();
        let base = line[..i].parse::<f64>().unwrap();
        let exp = line[i + 1..].parse::<f64>().unwrap();
        let val = exp * base.ln();
        if val > max_val {
            max_val = val;
            max_idx = idx;
        }
    }
    Ok(max_idx.to_string())
}

common::problem!("709", "p099_base_exp.txt", solve);
