#[link(name = "prob0099", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::rt::io;
use std::rt::io::buffered::BufferedReader;
use std::rt::io::file::FileInfo;
use common::reader::BufferedReaderUtil;

pub static EXPECTED_ANSWER: &'static str = "709";

pub fn solve() -> ~str {
    let r = Path::new("files/base_exp.txt").open_reader(io::Open).expect("file not found.");
    let mut br = BufferedReader::new(r);

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
