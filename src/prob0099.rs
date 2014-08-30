#![crate_name = "prob0099"]
#![crate_type = "rlib"]

use std::io::{BufferedReader, File};
use std::iter;

pub static EXPECTED_ANSWER: &'static str = "709";

pub fn solve() -> String {
    let mut br = BufferedReader::new(
        File::open(&Path::new("files/p099_base_exp.txt")).ok().expect("file not found."));

    br.lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            let line = line.as_slice().trim();
            let i = line.find(',').unwrap();
            let base = from_str::<f64>(line.slice(0, i)).unwrap();
            let exp  = from_str::<f64>(line.slice(i + 1, line.len())).unwrap();
            exp * base.ln()
        }).zip(iter::count(1u, 1))
        .fold(None::<(f64, uint)>, |max, (x, i)| {
            match max {
                None => Some((x, i)),
                Some((y, j)) => if x > y {
                    Some((x, i))
                } else {
                    Some((y, j))
                }
            }
        }).unwrap()
        .val1()
        .to_string()
}
