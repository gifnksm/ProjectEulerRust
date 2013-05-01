use core::iterator::{ IteratorUtil };

use std::sort::{ merge_sort };

use common::extiter::{ AdditiveIterator };
use common::reader::{ read_whole_word };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 22,
    answer: "871198282",
    solver: solve
};

fn get_score(n: uint, s: &str) -> uint {
    n * str::as_bytes_slice(s).map(|c| *c - ('A' as u8) + 1).foldl(0 as uint, |s, e| *s + *e as uint)
}

fn solve() -> ~str {
    let result = io::read_whole_file_str(&Path("files/names.txt")).chain(|input| {
        read_whole_word(input).map(|names| {
            merge_sort(*names, |a, b| a < b)
                .mapi(|i, s| get_score(i + 1, *s))
        })
    }).map(|scores| scores.iter().transform(|&x| x).sum());

    match result {
        result::Err(msg) => { fail!(fmt!("%s", msg)); }
        result::Ok(score) => { return score.to_str(); }
    }
}
