//! [Problem 22](https://projecteuler.net/problem=22) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
};

fn get_score(n: u32, s: &str) -> u32 {
    n * s.bytes().map(|c| (c - b'A' + 1) as u32).sum::<u32>()
}

fn compute(words: &[String]) -> u32 {
    let mut words = words
        .iter()
        .map(|word| word.trim().trim_matches('\"'))
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>();
    words.sort();
    words
        .into_iter()
        .enumerate()
        .map(|(i, s)| get_score((i + 1) as u32, s))
        .sum()
}

fn solve(file: File) -> io::Result<String> {
    let mut words = vec![];

    for bytes in BufReader::new(file).split(b',') {
        let mut bytes = bytes?;
        if bytes.last() == Some(&b',') {
            let _ = bytes.pop();
        }
        words.push(String::from_utf8(bytes).unwrap());
    }

    Ok(compute(&words).to_string())
}

common::problem!("871198282", "p022_names.txt", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn colin() {
        assert_eq!(49714, super::get_score(938, "COLIN"));
    }
}
