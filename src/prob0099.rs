#[link(name = "prob0099", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::io::buffered::BufferedReader;
use std::io::File;
use common::reader::BufferedReaderUtil;

pub static EXPECTED_ANSWER: &'static str = "709";

pub fn solve() -> ~str {
    let mut br = BufferedReader::new(
        File::open(&Path::new("files/base_exp.txt")).expect("file not found."));

    let mut line_idx = 1u;
    let mut max = 0.0;
    let mut max_idx = 1;
    for line in br.line_iter() {
        let line = line.trim();
        let opt = line.find(',');
        for &idx in opt.iter() {
            let base: f64 = from_str(line.slice(0, idx)).unwrap();
            let exp:  f64 = from_str(line.slice(idx + 1, line.len())).unwrap();
            let ln = exp * base.ln();
            if ln > max {
                max = ln;
                max_idx = line_idx;
            }
            line_idx += 1;
        }
    }
    max_idx.to_str()
}
