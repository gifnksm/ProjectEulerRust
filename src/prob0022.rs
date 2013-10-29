#[link(name = "prob0022", vers = "0.0")];
#[crate_type = "lib"];

extern mod extra;
extern mod common;

use std::rt::io;
use std::rt::io::buffered::BufferedReader;
use std::rt::io::file::FileInfo;
use std::iter::AdditiveIterator;
use extra::sort::Sort;
use common::reader::BufferedReaderUtil;

pub static EXPECTED_ANSWER: &'static str = "871198282";

fn get_score(n: uint, s: &str) -> uint {
    n * s.byte_iter().map(|c| (c - ('A' as u8) + 1) as uint).sum()
}

pub fn solve() -> ~str {
    let reader = Path::new("files/names.txt").open_reader(io::Open).expect("file not found.");
    let mut input = BufferedReader::new(reader);

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
