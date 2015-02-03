//! [Problem 42](https://projecteuler.net/problem=42) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(collections, io)]

#[macro_use(problem)] extern crate common;
extern crate seq;

use std::old_io::{BufferedReader, IoResult, File};
use std::iter;
use seq::TriangularNums;

fn word_to_value(word: &str) -> u32 {
    let mut value = 0;
    for b in word.bytes() {
        value += (b - ('A' as u8) + 1) as u32;
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
        let mut word = &word_str[];
        if word.is_empty() { break; }

        cont = if word.ends_with(",") {
            word = word.trim_right_matches(',');
            true
        } else {
            false
        };

        word = word.trim_matches('\"');
        values.push(word_to_value(word));
    }

    let max_value = *values.iter().max().unwrap();
    let mut is_tri = iter::repeat(false)
        .take((max_value + 1) as usize)
        .collect::<Vec<_>>();
    for t in TriangularNums::<u32>::new().take_while(|&t| t <= max_value) {
        is_tri[t as usize] = true;
    }
    Ok(values.iter().filter(|&&v| is_tri[v as usize]).count().to_string())
}

problem!("162", "p042_words.txt", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn word_to_value() {
        assert_eq!(55, super::word_to_value("SKY"));
    }
}
