//! [Problem 59](https://projecteuler.net/problem=59) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use std::{
    f64,
    fs::File,
    io::{self, prelude::*, BufReader},
};

const ENGLISH_FREQUENCY: &[(char, f64)] = &[
    ('a', 0.08167),
    ('b', 0.01492),
    ('c', 0.02782),
    ('d', 0.04253),
    ('e', 0.12702),
    ('f', 0.02228),
    ('g', 0.02015),
    ('h', 0.06094),
    ('i', 0.06966),
    ('j', 0.00153),
    ('k', 0.00772),
    ('l', 0.04025),
    ('m', 0.02406),
    ('n', 0.06749),
    ('o', 0.07507),
    ('p', 0.01929),
    ('q', 0.00095),
    ('r', 0.05987),
    ('s', 0.06327),
    ('t', 0.09056),
    ('u', 0.02758),
    ('v', 0.00978),
    ('w', 0.02360),
    ('x', 0.00150),
    ('y', 0.01974),
    ('z', 0.00074),
];

fn trans_map<T: Clone>(key: u8, src: &[T], dst: &mut [T]) {
    for (i, f) in src.iter().enumerate() {
        dst[((i as u8) ^ key) as usize] = f.clone();
    }
}

fn get_dist(a: &[f64], b: &[f64]) -> f64 {
    let mut sum = 0.0;
    for (&na, &nb) in a.iter().zip(b.iter()) {
        sum += (na - nb) * (na - nb);
    }
    sum
}

fn find_key(count: &[usize], ref_freq: &[f64]) -> u8 {
    let total = count.iter().sum::<usize>();

    let freq = &mut [0.0f64; 256];
    for (f, &n) in freq.iter_mut().zip(count.iter()) {
        *f = (n as f64) / (total as f64);
    }

    let freq_buf = &mut [0.0; 256];
    let mut min_key = 0;
    let mut min_dist = f64::INFINITY;
    for k in 0..=255 {
        trans_map(k, freq, freq_buf);
        let dist = get_dist(freq_buf, ref_freq);
        if dist < min_dist {
            min_dist = dist;
            min_key = k;
        }
    }
    min_key
}

fn read_file(file: File) -> io::Result<Vec<u8>> {
    let mut code_list = vec![];

    for word in BufReader::new(file).split(b',') {
        let word_str = String::from_utf8(word?).ok().unwrap();
        let word = word_str.trim();
        if word.is_empty() {
            break;
        }
        code_list.push(word.parse::<u8>().unwrap())
    }

    Ok(code_list)
}

fn solve(file: File) -> io::Result<String> {
    const KEY_LEN: usize = 3;
    let code_list = read_file(file)?;

    let freq_dict = &mut [0.0; 256];
    for &(c, f) in ENGLISH_FREQUENCY {
        freq_dict[(c as u8) as usize] = f;
    }

    let mut freq = [[0; 256]; KEY_LEN];
    for (i, &n) in code_list.iter().enumerate() {
        freq[i % KEY_LEN][n as usize] += 1;
    }

    let key = freq
        .iter()
        .map(|f| find_key(f, freq_dict))
        .collect::<Vec<u8>>();

    let sum = code_list
        .iter()
        .zip(key.iter().cycle())
        .map(|(&n, &key)| (n ^ key) as u32)
        .sum::<u32>();

    Ok(sum.to_string())
}

common::problem!("129448", "p059_cipher.txt", solve);
