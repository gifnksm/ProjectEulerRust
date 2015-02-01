//! [Problem 98](https://projecteuler.net/problem=98) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#[macro_use(problem)] extern crate common;
extern crate integer;

use std::{cmp, iter, mem, uint};
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::old_io::{BufferedReader, File, IoResult};
use std::num::Int;
use integer::Integer;

fn read_words(file: File) -> IoResult<Vec<String>> {
    let mut input = BufferedReader::new(file);
    let mut words = vec![];

    // FIXME: This should be rewritten by using new iterator adapter, such as
    // `Iterator<char>::split()`.
    let mut cont = true;
    while cont {
        let word_str = String::from_utf8(try!(input.read_until(b','))).ok().unwrap();
        let mut word = &word_str[];
        if word.is_empty() { break; }

        cont = if word.ends_with(",") {
            word = word.trim_right_matches(',');
            true
        } else {
            false
        };

        word = word.trim_matches('\"');

        words.push(word.to_string());
    }
    Ok(words)
}

fn get_anagram_groups(words: Vec<String>) -> Vec<Vec<String>> {
    let mut map = HashMap::new();

    for word in words.into_iter() {
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

    for mut group in groups.into_iter() {
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

fn get_indices_pairs(pairs: Vec<(String, String)>) -> Vec<(uint, Vec<uint>, Vec<uint>)> {
    pairs
        .into_iter()
        .map(|(w1, w2)| {
            let cs1 = w1.as_bytes();
            let cs2 = w2.as_bytes();
            let get_pos = |&: &c: &u8| cs1.position_elem(&c).unwrap();
            (w1.len(),
             cs1.iter().map(|c| get_pos(c)).collect(),
             cs2.iter().map(|c| get_pos(c)).collect())
        }).collect::<Vec<_>>()
}

fn group_by_len(mut indices: Vec<(uint, Vec<uint>, Vec<uint>)>) -> Vec<(uint, Vec<(Vec<uint>, Vec<uint>)>)> {
    let mut groups = vec![];
    let mut cur_len = uint::MAX;
    let mut cur_group = vec![];

    indices.sort_by(|&(l1, _, _), &(l2, _, _)| l2.cmp(&l1));

    for (len, v1, v2) in indices.into_iter() {
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

fn check_digit(idx: &[uint], ds: &[uint]) -> bool {
    for i in (0 .. idx.len()) {
        if ds[i] != ds[idx[i]] { return false; }
        if ds.position_elem(&ds[idx[i]]).unwrap() != idx[i] { return false; }
    }
    true
}

fn idx_to_num(idx: &[uint], ds: &[uint]) -> uint {
    idx.iter().fold(0u, |num, &i| 10 * num + ds[i])
}

fn is_square(n: uint) -> bool {
    let sq = n.sqrt();
    (sq * sq == n)
}

fn max_square(groups: Vec<(uint, Vec<(Vec<uint>, Vec<uint>)>)>) -> uint {
    let mut max = 0;

    for (len, pairs) in groups.into_iter() {
        let mut nums = vec![];

        let start = 10u.pow(len - 1);
        let end   = 10u.pow(len);

        let mut nmin = start.sqrt();
        while nmin * nmin < start { nmin += 1; }

        for n in iter::count(nmin, 1).take_while(|&n| n * n < end) {
            let ds = (n * n).into_digits(10).rev().collect::<Vec<_>>();
            for &(ref v1, ref v2) in pairs.iter() {
                if ds[v2[0]] == 0 { continue }
                if !check_digit(&v1[], &ds[]) { continue }
                let num2 = idx_to_num(&v2[], &ds[]);
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

fn solve(file: File) -> IoResult<String> {
    let words = try!(read_words(file));
    let groups = get_anagram_groups(words);
    let pairs = flatten_groups(groups);
    let indices = get_indices_pairs(pairs);
    let groups = group_by_len(indices);
    let max = max_square(groups);

    Ok(max.to_string())
}

problem!("18769", "p098_words.txt", solve);
