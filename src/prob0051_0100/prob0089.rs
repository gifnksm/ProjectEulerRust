#[link(name = "prob0089", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 89,
    answer: "743",
    solver: solve
};

static roman_pairs: &'static [(&'static str, uint)] = &[
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
    ("M", 1000)
];

fn from_roman(mut s: &str) -> Option<uint> {
    let mut last_d = uint::max_value;

    let mut n = 0;
    while !s.is_empty() {
        match roman_pairs.find(|&(ds, _d)| s.starts_with(ds)) {
            Some((ds, d)) => {
                if d > last_d { return None; }
                n += d;
                s = s.slice(ds.len(), s.len());
                last_d = d;
            }
            None => { return None; }
        }
    }

    return Some(n);
}

fn to_roman(mut n: uint) -> ~str {
    let mut s = ~"";
    while n >= 1000 { n -= 1000; s += "M"; }
    if n >= 900 { n -= 900; s += "CM"; }
    if n >= 500 { n -= 500; s += "D"; }
    if n >= 400 { n -= 400; s += "CD"; }
    while n >= 100 { n -= 100; s += "C"; }
    if n >= 90 { n -= 90; s += "XC"; }
    if n >= 50 { n -= 50; s += "L"; }
    if n >= 40 { n -= 40; s += "XL"; }
    while n >= 10 { n -= 10; s += "X"; }
    if n >= 9 { n -= 9; s += "IX"; }
    if n >= 5 { n -= 5; s += "V"; }
    if n >= 4 { n -= 4; s += "IV"; }
    while n > 0 { n -= 1; s += "I"; }
    return s;
}

pub fn solve() -> ~str {
    let result = io::file_reader(&Path("files/roman.txt")).map(|file| {
        let mut sum = 0;
        for file.each_line |line| {
            sum += line.len() - to_roman(from_roman(line).get()).len();
        }
        sum
    });

    match result {
        Err(msg) => fail!(msg),
        Ok(value) => return value.to_str()
    }
}
