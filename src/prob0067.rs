#[link(name = "prob0067", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;
use std::{uint, vec};
use std::io::buffered::BufferedReader;
use std::io::File;
use common::reader::BufferedReaderUtil;

pub static EXPECTED_ANSWER: &'static str = "7273";

pub fn solve() -> ~str {
    let mut br = BufferedReader::new(
        File::open(&Path::new("files/triangle.txt")).expect("file not found."));

    let triangle = br.line_iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.word_iter().filter_map(from_str::<uint>).to_owned_vec())
        .to_owned_vec();
    let init = triangle.init();
    let last = triangle.last();
    (do init.rev_iter().fold(last.to_owned()) |prev, elem| {
            do vec::from_fn(elem.len()) |i| {
                elem[i] + uint::max(prev[i], prev[i + 1])
            }
        })[0].to_str()
}

