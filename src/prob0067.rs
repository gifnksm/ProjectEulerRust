#[link(name = "prob0067", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;
use std::{uint, vec};
use std::rt::io;
use std::rt::io::buffered::BufferedReader;
use std::rt::io::file::FileInfo;
use common::reader::BufferedReaderUtil;

pub static EXPECTED_ANSWER: &'static str = "7273";

pub fn solve() -> ~str {
    let r = Path::new("files/triangle.txt").open_reader(io::Open).expect("file not found.");
    let mut br = BufferedReader::new(r);

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

