#[link(name = "prob0059", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;
use std::{uint, vec, float, u8, str, io};
use common::problem::{Problem};

pub static problem: Problem<'static> = Problem {
    id: 59,
    answer: "107359",
    solver: solve
};

static english_frequency: &'static [(char, float)] = &[
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
    ('z', 0.00074)
];

fn trans_map<T: Copy>(key: u8, src: &[T], dst: &mut [T]) {
    for src.eachi |i, &f| {
        dst[(i as u8) ^ key] = f;
    }
}

fn get_dist(a: &[float], b: &[float]) -> float {
    let mut sum = 0f;
    for vec::each2(a, b) |&na, &nb| {
        sum += (na - nb) * (na - nb);
    }
    return sum;
}

fn find_key(count: &[uint], ref_freq: &[float]) -> u8 {
    let total = count.foldl(0, |&s, &n| s + n);

    let mut freq = ~[0f, ..256];
    for count.eachi |i, &n| { freq[i] = (n as float) / (total as float) }

    let mut freq_buf = ~[0f, ..256];
    let mut min_key  = 0;
    let mut min_dist = float::infinity;
    for uint::range(0, 256) |k| {
        trans_map(k as u8, freq, freq_buf);
        let dist = get_dist(freq_buf, ref_freq);
        if dist < min_dist {
            min_dist = dist;
            min_key = k;
        }
    }
    return min_key as u8;
}

pub fn solve() -> ~str {
    let mut freq_dict = ~[0f, ..256];
    for english_frequency.each |&(c, f)| {
        freq_dict[c as u8] = f;
    }

    let result = io::read_whole_file_str(&Path("files/cipher1.txt"))
        .map(|&input| {
            let mut val = ~[];
            for str::each_split_char(input.trim(), ',') |n| {
                val.push(u8::from_str(n).get());
            }
            val
        }).map(|&input| {
            let mut freq = [~[0, ..256], ~[0, ..256], ~[0, ..256]];
            for input.eachi |i, &n| {
                freq[i % 3][n] += 1;
            }
            (freq.map(|&f| find_key(f, freq_dict)), input)
        }).map(|&(key, input)| {
            let l = key.len();
            do input.mapi |i, &n| { n ^ key[i % l] }
        }).map(|&input| {
            input.foldl(0u, |s, &n| s + (n as uint))
        });

    return match result {
        Ok(answer) => answer.to_str(),
        Err(e)     => fail!(e)
    };
}

