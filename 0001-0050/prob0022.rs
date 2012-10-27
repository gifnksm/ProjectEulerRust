use io::{WriterUtil};

extern mod std;
use std::sort::{ merge_sort };

extern mod euler;
use euler::reader::{ read_whole_word };

fn get_score(n: uint, s: &str) -> uint {
    n * str::as_bytes_slice(s).map(|c| *c - ('A' as u8) + 1).foldl(0 as uint, |s, e| *s + *e as uint)
}

fn main() {
    let result = do io::read_whole_file_str(&Path("files/names.txt")).chain |input| {
        do read_whole_word(input).map |names| {
            merge_sort(*names, |a, b| a < b).mapi(|i, s| get_score(i + 1, *s))
        }
    };
    match result {
        result::Err(msg) => {
            io::stderr().write_str(fmt!("%s\n", msg));
        }
        result::Ok(scores) => {
            let mut total_score = 0;
            for scores.each |s| { total_score += *s; }
            io::println(fmt!("answer: %u", total_score));
        }
    }
}