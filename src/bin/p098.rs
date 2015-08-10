//! [Problem 98](https://projecteuler.net/problem=98) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results)]

#[macro_use(problem)] extern crate common;
extern crate integer;

use std::{cmp, mem, u64};
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use integer::Integer;

fn read_words(file: File) -> io::Result<Vec<String>> {
    let mut words = vec![];

    for bytes in BufReader::new(file).split(b',') {
        let word_str = String::from_utf8(try!(bytes)).ok().unwrap();
        let word = word_str
            .trim_right_matches(',')
            .trim_matches('\"');
        words.push(word.to_string());
    }
    Ok(words)
}

fn get_anagram_groups(words: Vec<String>) -> Vec<Vec<String>> {
    let mut map = HashMap::new();

    for word in words {
        let mut cs = word.chars().collect::<Vec<_>>();
        cs.sort();
        match map.entry(cs) {
            Entry::Vacant(e) => {
                let _ = e.insert(vec![word]);
            }
            Entry::Occupied(e) => {
                let _ = e.into_mut().push(word);
            }
        }
    }

    map.into_iter()
        .map(|(_, vals)| vals)
        .filter(|vals| vals.len() > 1)
        .collect()
}

fn flatten_groups(groups: Vec<Vec<String>>) -> Vec<(String, String)> {
    let mut pairs = Vec::with_capacity(groups.len());

    for mut group in groups {
        if group.len() == 2 {
            pairs.push((group.remove(0), group.remove(0)));
            continue
        }

        for i in (0 .. group.len()) {
            for j in (i + 1 .. group.len()) {
                pairs.push((group[i].clone(), group[j].clone()))
            }
        }
    }

    pairs
}

fn get_indices_pairs(pairs: Vec<(String, String)>) -> Vec<(u64, Vec<u64>, Vec<u64>)> {
    pairs
        .into_iter()
        .map(|(w1, w2)| {
            let cs1 = w1.as_bytes();
            let cs2 = w2.as_bytes();
            let get_pos = |&c: &u8| cs1.iter().position(|&e| e == c).unwrap() as u64;
            (w1.len() as u64,
             cs1.iter().map(|c| get_pos(c)).collect(),
             cs2.iter().map(|c| get_pos(c)).collect())
        }).collect::<Vec<_>>()
}

fn group_by_len(mut indices: Vec<(u64, Vec<u64>, Vec<u64>)>) -> Vec<(u64, Vec<(Vec<u64>, Vec<u64>)>)> {
    let mut groups = vec![];
    let mut cur_len = u64::MAX;
    let mut cur_group = vec![];

    indices.sort_by(|&(l1, _, _), &(l2, _, _)| l2.cmp(&l1));

    for (len, v1, v2) in indices {
        if !cur_group.is_empty() && cur_len != len {
            groups.push((cur_len, mem::replace(&mut cur_group, vec![(v1, v2)])));
        } else {
            cur_group.push((v1, v2));
        }
        cur_len = len;
    }
    if !cur_group.is_empty() {
        groups.push((cur_len, cur_group));
    }
    groups
}

fn check_digit(idx: &[u64], ds: &[u64]) -> bool {
    for i in (0 .. idx.len()) {
        if ds[i] != ds[idx[i] as usize] {
            return false;
        }
        if ds.iter().position(|&e| e == ds[idx[i] as usize]).unwrap() as u64 != idx[i] {
            return false;
        }
    }
    true
}

fn idx_to_num(idx: &[u64], ds: &[u64]) -> u64 {
    idx.iter().fold(0, |num, &i| 10 * num + ds[i as usize])
}

fn is_square(n: u64) -> bool {
    let sq = n.sqrt();
    (sq * sq == n)
}

fn max_square(groups: Vec<(u64, Vec<(Vec<u64>, Vec<u64>)>)>) -> u64 {
    let mut max = 0;

    for (len, pairs) in groups {
        let mut nums = vec![];

        let start = 10u64.pow((len - 1) as u32);
        let end   = 10u64.pow(len as u32);

        let mut nmin = start.sqrt();
        while nmin * nmin < start { nmin += 1; }

        for n in (nmin..).take_while(|&n| n * n < end) {
            let ds = (n * n).into_digits(10).rev().collect::<Vec<_>>();
            for &(ref v1, ref v2) in &pairs {
                if ds[v2[0] as usize] == 0 { continue }
                if !check_digit(&v1, &ds) { continue }
                let num2 = idx_to_num(&v2, &ds);
                if !is_square(num2) { continue }
                nums.push(n * n);
            if n * n != num2 { nums.push(num2); }
            }
        }

        if !nums.is_empty() {
            max = cmp::max(nums.into_iter().max().unwrap(), max);
        }
    }

    max
}

fn solve(file: File) -> io::Result<String> {
    let words = try!(read_words(file));
    let groups = get_anagram_groups(words);
    let pairs = flatten_groups(groups);
    let indices = get_indices_pairs(pairs);
    let groups = group_by_len(indices);
    let max = max_square(groups);

    Ok(max.to_string())
}

problem!("18769", "p098_words.txt", solve);
