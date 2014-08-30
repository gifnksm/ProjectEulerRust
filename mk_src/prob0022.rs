#![crate_name = "prob0022"]
#![crate_type = "rlib"]

extern crate common;

use std::io::{BufferedReader, File};
use std::iter::AdditiveIterator;
use common::reader::BufferedReaderUtil;

pub static EXPECTED_ANSWER: &'static str = "871198282";

fn get_score(n: uint, s: &str) -> uint {
    n * s.bytes().map(|c| (c - ('A' as u8) + 1) as uint).sum()
}

pub fn solve() -> String {
    let mut input = BufferedReader::new(File::open(&Path::new("files/p022_names.txt"))
                                        .ok()
                                        .expect("file not found."));
    let mut ss = input.sep_iter(',' as u8)
        .map(|s| s.as_slice().trim().trim_chars('\"').to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>();
    ss.sort();
    ss.iter()
        .enumerate()
        .map(|(i, s)| { get_score(i + 1, s.as_slice())} )
        .sum()
        .to_string()
}
