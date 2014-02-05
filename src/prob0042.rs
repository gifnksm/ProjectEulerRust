#[crate_id = "prob0042"];
#[crate_type = "rlib"];

extern mod common;
extern mod math;

use std::{str, vec};
use std::io::File;
use common::reader;
use math::sequence;

pub static EXPECTED_ANSWER: &'static str = "162";

fn word_value(word: &str) -> uint {
    let mut value = 0;
    for b in word.bytes() {
        value += (b - ('A' as u8) + 1) as uint;
    }
    return value;
}

pub fn solve() -> ~str {
    let mut reader = File::open(&Path::new("files/words.txt"))
        .ok().expect("file not found.");
    let input = str::from_utf8_owned(reader.read_to_end().ok().unwrap()).unwrap();
    let result = reader::read_whole_word(input).map(|words| words.map(|w| word_value(*w)))
        .map(|values| {
            let mut is_tri = vec::from_elem(values.iter().max().unwrap() + 1, false);
            let mut it = sequence::triangle::<uint>().take_while(|&t| t < is_tri.len());
            for t in it { is_tri[t] = true; }

            values.iter().count(|&v| is_tri[v]).to_str()
        });

    match result {
        Err(msg) => { fail!(msg) }
        Ok(cnt) => { return cnt.to_str(); }
    }
}
