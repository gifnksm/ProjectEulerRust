#[crate_type = "lib"];

extern mod common;

use std::io::buffered::BufferedReader;
use std::io::File;
use std::iter;

pub static EXPECTED_ANSWER: &'static str = "709";

pub fn solve() -> ~str {
    let mut br = BufferedReader::new(
        File::open(&Path::new("files/base_exp.txt")).expect("file not found."));

    br.lines()
        .map(|line| {
            let line = line.trim();
            let i = line.find(',').unwrap();
            let base = from_str::<f64>(line.slice(0, i)).unwrap();
            let exp  = from_str::<f64>(line.slice(i + 1, line.len())).unwrap();
            exp * base.ln()
        }).zip(iter::count(1, 1))
        .max_by(|&(ln, _)| ln)
        .unwrap()
        .n1()
        .to_str()
}
