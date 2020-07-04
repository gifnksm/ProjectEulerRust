//! [Problem 79](https://projecteuler.net/problem=79) solver.

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
use topological_sort::TopologicalSort;

fn solve(file: File) -> io::Result<String> {
    let mut ts = TopologicalSort::new();
    for line in BufReader::new(file).lines() {
        let line = line?;
        let ds = line.trim().chars();
        for (prec, succ) in ds.clone().zip(ds.skip(1)) {
            ts.add_dependency(prec, succ);
        }
    }
    let s = ts
        .map(|d: char| d.to_string())
        .collect::<Vec<String>>()
        .concat();
    Ok(s)
}

common::problem!("73162890", "p079_keylog.txt", solve);
