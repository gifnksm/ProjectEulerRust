use std::sort::{ merge_sort };

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
    let result = do io::read_whole_file_str(&Path("files/names.txt")).chain |input| {
        do read_whole_word(input).map |names| {
            merge_sort(*names, |a, b| a < b).mapi(|i, s| get_score(i + 1, *s))
        }
    };
    match result {
        result::Err(msg) => {
            fail!(fmt!("%s", msg));
        }
        result::Ok(scores) => {
            let mut total_score = 0;
            for scores.each |s| { total_score += *s; }
            return total_score.to_str();
        }
    }
}