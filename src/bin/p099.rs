#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(phase, slicing_syntax)]

#[phase(plugin, link)] extern crate common;

use std::num::Float;
use std::io::{BufferedReader, File, IoResult};
use std::iter;

fn solve(file: File) -> IoResult<String> {
    let mut br = BufferedReader::new(file);
    let mut max_val = 0.0;
    let mut max_idx = 0;

    for (line, idx) in br.lines().zip(iter::count(1u, 1)) {
        let line = try!(line);
        let line = line.trim();
        let i = line.find(',').unwrap();
        let base = line[.. i].parse::<f64>().unwrap();
        let exp  = line[i + 1 ..].parse::<f64>().unwrap();
        let val = exp * base.ln();
        if val > max_val {
            max_val = val;
            max_idx = idx;
        }
    }
    Ok(max_idx.to_string())
}

problem!("709", "p099_base_exp.txt", solve);
