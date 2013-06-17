#[link(name = "prob0042", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::{io, result, vec};
use common::extiter::Triangle;
use common::reader;
use common::problem::Problem;

pub static problem: Problem<'static> = Problem {
    id: 42,
    answer: "162",
    solver: solve
};

fn word_value(word: &str) -> uint {
    let mut value = 0;
    for word.bytes_iter().advance |b| {
        value += (b - ('A' as u8) + 1) as uint;
    }
    return value;
}

pub fn solve() -> ~str {
    let result = io::read_whole_file_str(&Path("files/words.txt")).chain(|input| {
        do reader::read_whole_word(input).map |words| { words.map(|w| word_value(*w)) }
    }).map(|values| {
        let mut is_tri = vec::from_elem(values.iter().max().unwrap() + 1, false);
        let mut it = Triangle::new().take_while(|&t| t < is_tri.len());
        for it.advance() |t| { is_tri[t] = true; }

        let mut cnt = 0u;
        for values.each |&v| { if is_tri[v] { cnt += 1; } }
        cnt
    });
    match result {
        result::Err(msg) => { fail!(msg) }
        result::Ok(cnt) => { return cnt.to_str(); }
    }
}
