//! [Problem 17](https://projecteuler.net/problem=17) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

fn to_word_under10(n: u32) -> String {
    match n {
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
        _ => panic!(),
    }
}

fn to_word_under20(n: u32) -> String {
    assert!(n < 20);
    if n < 10 {
        return to_word_under10(n);
    }
    match n {
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
        _ => panic!(),
    }
}

fn to_word_under100(n: u32) -> String {
    assert!(n < 100);
    if n < 20 {
        return to_word_under20(n);
    }

    let prefix = match n / 10 {
        0 | 1 => panic!(),
        2 => "twenty".to_string(),
        3 => "thirty".to_string(),
        4 => "forty".to_string(),
        5 => "fifty".to_string(),
        6 => "sixty".to_string(),
        7 => "seventy".to_string(),
        8 => "eighty".to_string(),
        9 => "ninety".to_string(),
        _ => panic!(),
    };
    if n % 10 != 0 {
        format!("{}-{}", prefix, to_word_under10(n % 10))
    } else {
        prefix
    }
}

fn to_word_under1000(n: u32) -> String {
    assert!(n < 1000);
    if n < 100 {
        return to_word_under100(n);
    }

    let prefix = format!("{} hundred", to_word_under10(n / 100));
    if n % 100 != 0 {
        format!("{} and {}", prefix, to_word_under100(n % 100))
    } else {
        prefix
    }
}

fn to_word(n: u32) -> String {
    assert!(n <= 1000);
    if n < 1000 {
        return to_word_under1000(n);
    }
    "one thousand".to_string()
}

fn compute(max: u32) -> u32 {
    (1..max + 1)
        .map(to_word)
        .map(|w| w.chars().filter(|&c| c != '-' && c != ' ').count() as u32)
        .sum()
}

fn solve() -> String {
    compute(1000).to_string()
}

common::problem!("21124", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn one_to_five() {
        assert_eq!(19, super::compute(5));
    }
}
