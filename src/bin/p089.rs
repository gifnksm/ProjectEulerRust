//! [Problem 89](https://projecteuler.net/problem=89) solver.

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
    u32,
};

const ROMAN_PAIRS: &[(&str, u32)] = &[
    ("IV", 4),
    ("IX", 9),
    ("XL", 40),
    ("XC", 90),
    ("CD", 400),
    ("CM", 900),
    ("I", 1),
    ("V", 5),
    ("X", 10),
    ("L", 50),
    ("C", 100),
    ("D", 500),
    ("M", 1000),
];

fn from_roman(mut s: &str) -> Option<u32> {
    let mut last_d = u32::MAX;

    let mut n = 0;
    while !s.is_empty() {
        match ROMAN_PAIRS.iter().find(|&&(ds, _d)| s.starts_with(ds)) {
            Some(&(ds, d)) => {
                if d > last_d {
                    return None;
                }
                n += d;
                s = &s[ds.len()..];
                last_d = d;
            }
            None => {
                return None;
            }
        }
    }

    Some(n)
}

fn to_roman(mut n: u32) -> String {
    let mut s = String::new();
    while n >= 1000 {
        n -= 1000;
        s.push_str("M");
    }
    if n >= 900 {
        n -= 900;
        s.push_str("CM");
    }
    if n >= 500 {
        n -= 500;
        s.push_str("D");
    }
    if n >= 400 {
        n -= 400;
        s.push_str("CD");
    }
    while n >= 100 {
        n -= 100;
        s.push_str("C");
    }
    if n >= 90 {
        n -= 90;
        s.push_str("XC");
    }
    if n >= 50 {
        n -= 50;
        s.push_str("L");
    }
    if n >= 40 {
        n -= 40;
        s.push_str("XL");
    }
    while n >= 10 {
        n -= 10;
        s.push_str("X");
    }
    if n >= 9 {
        n -= 9;
        s.push_str("IX");
    }
    if n >= 5 {
        n -= 5;
        s.push_str("V");
    }
    if n >= 4 {
        n -= 4;
        s.push_str("IV");
    }
    while n > 0 {
        n -= 1;
        s.push_str("I");
    }
    s
}

fn solve(file: File) -> io::Result<String> {
    let mut sum = 0;
    for line in BufReader::new(file).lines() {
        let line = line?;
        let line = line.trim();
        sum += line.len() - to_roman(from_roman(line).unwrap()).len();
    }
    Ok(sum.to_string())
}

common::problem!("743", "p089_roman.txt", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn from_roman() {
        assert_eq!(Some(16), super::from_roman("IIIIIIIIIIIIIIII"));
        assert_eq!(Some(16), super::from_roman("VIIIIIIIIIII"));
        assert_eq!(Some(16), super::from_roman("VVIIIIII"));
        assert_eq!(Some(16), super::from_roman("XIIIIII"));
        assert_eq!(Some(16), super::from_roman("VVVI"));
        assert_eq!(Some(16), super::from_roman("XVI"));
    }

    #[test]
    fn to_roman() {
        assert_eq!("XVI", super::to_roman(16));
    }
}
