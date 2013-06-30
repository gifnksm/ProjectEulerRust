#[link(name = "prob0022", vers = "0.0")];
#[crate_type = "lib"];

extern mod extra;
extern mod common;

use std::{io, result};
use std::iterator::AdditiveIterator;
use extra::sort;
use common::reader;

pub static expected_answer: &'static str = "871198282";

fn get_score(n: uint, s: &str) -> uint {
    n * s.bytes_iter().transform(|c| (c - ('A' as u8) + 1) as uint).sum()
}

pub fn solve() -> ~str {
    let result = io::read_whole_file_str(&Path("files/names.txt")).chain(|input| {
        reader::read_whole_word(input)
            .map(|&names| names.map(|s| s.to_str()))
            .map(|&names| sort::merge_sort(names, |a, b| a < b).mapi(|i, &s| get_score(i + 1, s)))
    }).map(|scores| scores.iter().transform(|&x| x).sum());

    match result {
        result::Err(msg) => { fail!(fmt!("%s", msg)); }
        result::Ok(score) => { return score.to_str(); }
    }
}
