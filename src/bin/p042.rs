//! [Problem 42](https://projecteuler.net/problem=42) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use seq::TriangularNums;
use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
};

fn word_to_value(word: &str) -> u32 {
    let mut value = 0;
    for b in word.bytes() {
        value += (b - b'A' + 1) as u32;
    }
    value
}

fn solve(file: File) -> io::Result<String> {
    let mut values = vec![];

    for bytes in BufReader::new(file).split(b',') {
        let word_str = String::from_utf8(bytes?).unwrap();
        let word = word_str.trim_end_matches(',').trim_matches('\"');
        values.push(word_to_value(word));
    }

    let max_value = *values.iter().max().unwrap();
    let mut is_tri = vec![false; (max_value + 1) as usize];
    for t in TriangularNums::<u32>::new().take_while(|&t| t <= max_value) {
        is_tri[t as usize] = true;
    }
    Ok(values
        .iter()
        .filter(|&&v| is_tri[v as usize])
        .count()
        .to_string())
}

common::problem!("162", "p042_words.txt", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn word_to_value() {
        assert_eq!(55, super::word_to_value("SKY"));
    }
}
