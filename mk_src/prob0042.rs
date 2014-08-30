#![crate_name = "prob0042"]
#![crate_type = "rlib"]

extern crate common;
extern crate math;

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

pub fn solve() -> String {
    let mut reader = File::open(&Path::new("files/p042_words.txt"))
        .ok().expect("file not found.");
    let input = String::from_utf8(reader.read_to_end().ok().unwrap()).unwrap();
    let result = reader::read_whole_word(input.as_slice())
        .map(|words| words.iter().map(|w| word_value(*w)).collect::<Vec<uint>>())
        .map(|values| {
            let len = values.iter().max().unwrap() + 1;
            let mut is_tri = Vec::from_elem(len, false);
            let mut it = sequence::triangle::<uint>().take_while(|&t| t < len);
            for t in it { *is_tri.get_mut(t) = true; }

            values.iter().filter(|&&v| is_tri[v]).count().to_string()
        });

    match result {
        Err(msg) => { fail!(msg) }
        Ok(cnt) => { cnt.to_string() }
    }
}
