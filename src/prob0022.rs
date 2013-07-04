#[link(name = "prob0022", vers = "0.0")];
#[crate_type = "lib"];

extern mod extra;
extern mod common;

use std::{io, result};
use std::iterator::AdditiveIterator;
use extra::sort::Sort;
use common::reader::ReaderIterator;

pub static EXPECTED_ANSWER: &'static str = "871198282";

fn get_score(n: uint, s: &str) -> uint {
    n * s.bytes_iter().transform(|c| (c - ('A' as u8) + 1) as uint).sum()
}

pub fn solve() -> ~str {
    let result = io::file_reader(&Path("files/names.txt")).map(|input| {
        let mut ss = input.sep_iter(',' as u8, false)
            .transform(|s| s.trim().trim_chars(&'"').to_str())
            .collect::<~[~str]>();
        ss.qsort();
        ss.iter()
            .enumerate()
            .transform(|(i, &s)| {  get_score(i + 1, s)} )
            .sum()
    });

    match result {
        result::Err(msg) => { fail!(fmt!("%s", msg)); }
        result::Ok(score) => { return score.to_str(); }
    }
}
