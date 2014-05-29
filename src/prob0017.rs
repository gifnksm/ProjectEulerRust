#![crate_id = "prob0017"]
#![crate_type = "rlib"]

use std::iter::AdditiveIterator;

pub static EXPECTED_ANSWER: &'static str = "21124";

fn to_word_under10(n: uint) -> String {
    return match n {
        0 => "zero".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        _ => fail!()
    };
}

fn to_word_under20(n: uint) -> String {
    assert!(n < 20);
    if n < 10 { return to_word_under10(n); }
    return match n {
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        _  => fail!()
    };
}

fn to_word_under100(n: uint) -> String {
    assert!(n < 100);
    if n < 20 { return to_word_under20(n); }

    let prefix = match n / 10 {
        0 | 1 => fail!(),
        2 => "twenty".to_string(),
        3 => "thirty".to_string(),
        4 => "forty".to_string(),
        5 => "fifty".to_string(),
        6 => "sixty".to_string(),
        7 => "seventy".to_string(),
        8 => "eighty".to_string(),
        9 => "ninety".to_string(),
        _ => fail!()
    };
    if n % 10 != 0 {
        format!("{}-{}", prefix, to_word_under10(n % 10))
    } else {
        prefix
    }
}

fn to_word_under1000(n: uint) -> String {
    assert!(n < 1000);
    if n < 100 { return to_word_under100(n); }

    let prefix = format!("{} hundred", to_word_under10(n / 100));
    if n % 100 != 0 {
        format!("{} and {}", prefix, to_word_under100(n % 100))
    } else {
        prefix
    }
}

fn to_word(n: uint) -> String {
    assert!(n <= 1000);
    if n < 1000 { return to_word_under1000(n); }
    return "one thousand".to_string();
}

pub fn solve() -> String {
    range(1u, 1001)
        .map(to_word)
        .map(|w| w.as_slice()
             .chars()
             .filter(|&c| c != '-' && c != ' ')
             .len())
        .sum()
        .to_str()
}
