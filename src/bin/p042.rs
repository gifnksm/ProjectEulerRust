#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(slicing_syntax)]

extern crate common;
extern crate seq;

use std::io::{BufferedReader, IoResult, File};
use common::Solver;
use seq::TriangularNums;

fn word_to_value(word: &str) -> uint {
    let mut value = 0;
    for b in word.bytes() {
        value += (b - ('A' as u8) + 1) as uint;
    }
    value
}

fn solve(file: File) -> IoResult<String> {
    let mut input = BufferedReader::new(file);
    let mut values = vec![];

    // FIXME: This should be rewritten by using new iterator adapter, such as
    // `Iterator<char>::split()`.
    let mut cont = true;
    while cont {
        let word_str = String::from_utf8(try!(input.read_until(b','))).ok().unwrap();
        let mut word = word_str[];
        if word.is_empty() { break; }

        cont = if word.ends_with(",") {
            word = word.trim_right_chars(',');
            true
        } else {
            false
        };

        word = word.trim_chars('\"');
        values.push(word_to_value(word));
    }

    let max_value = *values.iter().max().unwrap();
    let mut is_tri = Vec::from_elem(max_value + 1, false);
    for t in TriangularNums::<uint>::new().take_while(|&t| t <= max_value) {
        is_tri[t] = true;
    }
    Ok(values.iter().filter(|&&v| is_tri[v]).count().to_string())
}

fn main() { Solver::new_with_file("162", "p042_words.txt", solve).run(); }

#[cfg(test)]
mod tests {
    #[test]
    fn word_to_value() {
        assert_eq!(55, super::word_to_value("SKY"));
    }
}
