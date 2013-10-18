#[link(name = "prob0067", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;
use std::{uint, vec, io};
use common::reader::ReaderIterator;

pub static EXPECTED_ANSWER: &'static str = "7273";

pub fn solve() -> ~str {
    let result = io::file_reader(&Path::new("files/triangle.txt"))
        .map(|file| {
            let triangle = file.line_iter()
                .filter(|line| !line.is_empty())
                .map(|line| line.word_iter().filter_map(from_str::<uint>).to_owned_vec())
                .to_owned_vec();
            let init = triangle.init();
            let last = triangle.last();
            (do init.rev_iter().fold(last.to_owned()) |prev, elem| {
                    do vec::from_fn(elem.len()) |i| {
                        elem[i] + uint::max(prev[i], prev[i + 1])
                    }
                })[0]
        });

    match result {
        Err(msg) => fail!(msg),
        Ok(value) => return value.to_str()
    }
}

