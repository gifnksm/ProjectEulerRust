#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

extern crate common;
extern crate tsort;

use std::io::{BufferedReader, File, IoResult};
use common::Solver;
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

fn main() { Solver::new_with_file("73162890", "p079_keylog.txt", solve).run(); }
