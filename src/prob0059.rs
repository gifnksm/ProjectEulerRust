#[link(name = "prob0059", vers = "0.0")];
#[crate_type = "lib"];

use std::{float, u8, io};
use std::iterator::AdditiveIterator;

pub static EXPECTED_ANSWER: &'static str = "107359";

static ENGLISH_FREQUENCY: &'static [(char, float)] = &[
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

fn trans_map<T: Clone>(key: u8, src: &[T], dst: &mut [T]) {
    for (i, f) in src.iter().enumerate() {
        dst[(i as u8) ^ key] = f.clone();
    }
}

fn get_dist(a: &[float], b: &[float]) -> float {
    let mut sum = 0f;
    for (&na, &nb) in a.iter().zip(b.iter()) {
        sum += (na - nb) * (na - nb);
    }
    return sum;
}

fn find_key(count: &[uint], ref_freq: &[float]) -> u8 {
    let total = count.iter().map(|&x| x).sum();

    let mut freq = ~[0f, ..256];
    for (f, &n) in freq.mut_iter().zip(count.iter()) {
        *f = (n as float) / (total as float);
    }

    let mut freq_buf = ~[0f, ..256];
    let mut min_key  = 0;
    let mut min_dist = float::infinity;
    for k in range(0u, 256) {
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
    for &(c, f) in ENGLISH_FREQUENCY.iter() {
        freq_dict[c as u8] = f;
    }

    let result = io::read_whole_file_str(&Path("files/cipher1.txt"))
        .map(|input| {
            let code_list = input.trim().split_iter(',')
                .filter_map(u8::from_str).collect::<~[u8]>();

            let mut freq = [~[0u, ..256], ~[0u, ..256], ~[0u, ..256]];
            for (i, &n) in code_list.iter().enumerate() { freq[i % 3][n] += 1; }

            let keys = freq.map(|f| find_key(f.clone(), freq_dict));
            let l = keys.len();
            code_list.iter().enumerate()
                .map(|(i, &n)| (n ^ keys[i % l]) as uint)
                .sum()
        });

    return match result {
        Ok(answer) => answer.to_str(),
        Err(e)     => fail!(e)
    };
}

