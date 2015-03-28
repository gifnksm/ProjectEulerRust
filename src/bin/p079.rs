//! [Problem 79](https://projecteuler.net/problem=79) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results)]

#[macro_use(problem)] extern crate common;
extern crate topological_sort as tsort;

use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use tsort::TopologicalSort;

fn solve(file: File) -> io::Result<String> {
    let mut ts = TopologicalSort::new();
    for line in BufReader::new(file).lines() {
        let line = try!(line);
        let ds = line.trim().chars();
        for (prec, succ) in ds.clone().zip(ds.skip(1)) {
            ts.add_dependency(prec, succ);
        }
    }
    let s = ts.map(|d| d.to_string()).collect::<Vec<String>>().concat();
    Ok(s)
}

problem!("73162890", "p079_keylog.txt", solve);
