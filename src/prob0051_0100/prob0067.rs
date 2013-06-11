#[link(name = "prob0067", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::iterator::{IteratorUtil};
use std::{uint, vec, io};
use common::problem::{Problem};

pub static problem: Problem<'static> = Problem {
    id: 67,
    answer: "7273",
    solver: solve
};

pub fn solve() -> ~str {
    let result = io::file_reader(&Path("files/triangle.txt")).map(|file| {
        let mut triangle = ~[];
        for file.each_line |line| {
            triangle.push(line.word_iter().filter_map(uint::from_str).collect::<~[uint]>())
        }
        triangle
    }).map(|triangle| {
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
