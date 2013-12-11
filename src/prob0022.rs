#[crate_type = "lib"];

extern mod extra;
extern mod common;

use std::io::buffered::BufferedReader;
use std::io::File;
use std::iter::AdditiveIterator;
use extra::sort::Sort;
use common::reader::BufferedReaderUtil;

pub static EXPECTED_ANSWER: &'static str = "871198282";

fn get_score(n: uint, s: &str) -> uint {
    n * s.bytes().map(|c| (c - ('A' as u8) + 1) as uint).sum()
}

pub fn solve() -> ~str {
    let mut input = BufferedReader::new(File::open(&Path::new("files/names.txt"))
                                        .expect("file not found."));
    let mut ss = input.sep_iter(',' as u8)
        .map(|s| s.trim_chars(&',').trim().trim_chars(&'\"').to_str())
        .to_owned_vec();
    ss.qsort();
    ss.iter()
        .enumerate()
        .map(|(i, s)| {  get_score(i + 1, *s)} )
        .sum()
        .to_str()
}
