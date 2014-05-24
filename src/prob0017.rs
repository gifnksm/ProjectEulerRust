#![crate_id = "prob0017"]
#![crate_type = "rlib"]

use std::iter::AdditiveIterator;

pub static EXPECTED_ANSWER: &'static str = "21124";

fn to_word_under10(n: uint) -> StrBuf {
    return match n {
        0 => "zero".to_owned(),
        1 => "one".to_owned(),
        2 => "two".to_owned(),
        3 => "three".to_owned(),
        4 => "four".to_owned(),
        5 => "five".to_owned(),
        6 => "six".to_owned(),
        7 => "seven".to_owned(),
        8 => "eight".to_owned(),
        9 => "nine".to_owned(),
        _ => fail!()
    };
}

fn to_word_under20(n: uint) -> StrBuf {
    assert!(n < 20);
    if n < 10 { return to_word_under10(n); }
    return match n {
        10 => "ten".to_owned(),
        11 => "eleven".to_owned(),
        12 => "twelve".to_owned(),
        13 => "thirteen".to_owned(),
        14 => "fourteen".to_owned(),
        15 => "fifteen".to_owned(),
        16 => "sixteen".to_owned(),
        17 => "seventeen".to_owned(),
        18 => "eighteen".to_owned(),
        19 => "nineteen".to_owned(),
        _  => fail!()
    };
}

fn to_word_under100(n: uint) -> StrBuf {
    assert!(n < 100);
    if n < 20 { return to_word_under20(n); }

    let prefix = match n / 10 {
        0 | 1 => fail!(),
        2 => "twenty".to_owned(),
        3 => "thirty".to_owned(),
        4 => "forty".to_owned(),
        5 => "fifty".to_owned(),
        6 => "sixty".to_owned(),
        7 => "seventy".to_owned(),
        8 => "eighty".to_owned(),
        9 => "ninety".to_owned(),
        _ => fail!()
    };
    if n % 10 != 0 {
        format!("{}-{}", prefix, to_word_under10(n % 10))
    } else {
        prefix
    }
}

fn to_word_under1000(n: uint) -> StrBuf {
    assert!(n < 1000);
    if n < 100 { return to_word_under100(n); }

    let prefix = format!("{} hundred", to_word_under10(n / 100));
    if n % 100 != 0 {
        format!("{} and {}", prefix, to_word_under100(n % 100))
    } else {
        prefix
    }
}

fn to_word(n: uint) -> StrBuf {
    assert!(n <= 1000);
    if n < 1000 { return to_word_under1000(n); }
    return "one thousand".to_owned();
}

pub fn solve() -> StrBuf {
    range(1u, 1001)
        .map(to_word)
        .map(|w| w.as_slice()
             .chars()
             .filter(|&c| c != '-' && c != ' ')
             .len())
        .sum()
        .to_str()
}
