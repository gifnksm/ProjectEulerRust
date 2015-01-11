//! [Problem 79](https://projecteuler.net/problem=79) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#[macro_use(problem)] extern crate common;
extern crate "topological-sort" as tsort;

use std::io::{BufferedReader, File, IoResult};
use tsort::TopologicalSort;

fn solve(file: File) -> IoResult<String> {
    let mut br = BufferedReader::new(file);

    let mut ts = TopologicalSort::new();
    for line in br.lines() {
        let line = try!(line);
        let ds = line.trim().chars();
        for (prec, succ) in ds.zip(ds.skip(1)) {
            ts.add_dependency(prec, succ);
        }
    }
    let s = ts.map(|d| d.to_string()).collect::<Vec<String>>().concat();
    Ok(s)
}

problem!("73162890", "p079_keylog.txt", solve);
