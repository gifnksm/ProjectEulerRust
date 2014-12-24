#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(phase, slicing_syntax)]

#[phase(plugin, link)] extern crate common;

use std::io::{BufferedReader, IoResult, File};
use std::iter::AdditiveIterator;

fn get_score(n: uint, s: &str) -> uint {
    n * s.bytes().map(|c| (c - ('A' as u8) + 1) as uint).sum()
}

fn compute(words: &[String]) -> uint {
    let mut words = words.iter()
        .map(|word| word[].trim().trim_chars('\"'))
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>();
    words.sort();
    words.into_iter()
        .enumerate()
        .map(|(i, s)| get_score(i + 1, s))
        .sum()
}

fn solve(file: File) -> IoResult<String> {
    let mut input = BufferedReader::new(file);
    let mut words = vec![];

    // FIXME: This should be rewritten by using new iterator adapter, such as
    // `Iterator<char>::split()`.
    loop {
        let mut bytes = try!(input.read_until(b','));
        if bytes.is_empty() { break; }
        if bytes.last() == Some(&b',') {
            let _ = bytes.pop();
            words.push(String::from_utf8(bytes).unwrap());
        } else {
            words.push(String::from_utf8(bytes).unwrap());
            break;
        }
    }

    Ok(compute(words[]).to_string())
}

problem!("871198282", "p022_names.txt", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn colin() {
        assert_eq!(49714, super::get_score(938, "COLIN"));
    }
}
