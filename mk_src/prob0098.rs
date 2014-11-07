#![crate_name = "prob0098"]
#![crate_type = "rlib"]

extern crate common;
extern crate math;

use std::{num, mem, uint};
use std::iter::OrdIterator;
use std::io::File;
use std::collections::HashMap;
use common::reader;
use math::{arith, numconv};

pub const EXPECTED_ANSWER: &'static str = "18769";

#[inline(always)]
fn check_digit(idx: &[uint], ds: &[uint]) -> bool {
    for i in range(0, idx.len()) {
        if ds[i] != ds[idx[i]] { return false; }
        if ds.position_elem(&ds[idx[i]]).unwrap() != idx[i] { return false; }
    }
    true
}

#[inline(always)]
fn idx_to_num(idx: &[uint], ds: &[uint]) -> uint {
    idx.iter().fold(0u, |num, &i| 10 * num + ds[i])
}

#[inline(always)]
fn is_square(n: uint) -> bool {
    let sq = arith::isqrt(n);
    (sq * sq == n)
}

pub fn solve() -> String {
    let mut reader = File::open(&Path::new("files/p098_words.txt")).ok().expect("file not found.");
    let input = String::from_utf8(reader.read_to_end().ok().unwrap()).unwrap();

    let result = reader::read_whole_word(input.as_slice()).map(|words| {
        let mut map = HashMap::new();
        for &word in words.iter() {
            let mut cs = word.chars().collect::<Vec<char>>();
            cs.sort();
            match map.remove(&cs) {
                None => { map.insert(cs, vec!(word.to_string())); }
                Some(mut ws) => {
                    ws.push(word.to_string());
                    map.insert(cs, ws);
                }
            }
        }
        let mut buf = Vec::new();
        for (_key, values) in map.into_iter() {
            if values.len() > 1 {
                buf.push(values)
            }
        }
        buf
    }).map(|words| {
        let mut buf = Vec::with_capacity(words.len());
        for elt in words.iter() {
            for i in range(0, elt.len()) {
                for j in range(i + 1, elt.len()) {
                    buf.push((elt[i].clone(), elt[j].clone()))
                }
            }
        }
        buf
    }).map(|word_pairs| {
        let mut words = word_pairs.iter().map(|&(ref w1, ref w2)| {
            let cs1 = w1.as_bytes();
            let cs2 = w2.as_bytes();
            let get_pos = |&c: &u8| cs1.position_elem(&c).unwrap();
            (w1.len(),
             cs1.iter().map(|c| get_pos(c)).collect(),
             cs2.iter().map(|c| get_pos(c)).collect())
        }).collect::<Vec<(uint, Vec<uint>, Vec<uint>)>>();
        words.sort_by(|&(l1, _, _), &(l2, _, _)| l2.cmp(&l1));
        words
    }).map(|idx_pairs| {
        let mut buf = Vec::new();
        let mut cur_len = uint::MAX;
        let mut cur_group = Vec::new();
        for &(ref len, ref v1, ref v2) in idx_pairs.iter() {
            if cur_group.is_empty() || cur_len == *len {
                cur_len = *len;
                cur_group.push((v1.clone(), v2.clone()));
            } else {
                buf.push((cur_len, mem::replace(&mut cur_group, vec!((v1.clone(), v2.clone())))));
                cur_len = *len;
            }
        }
        if !cur_group.is_empty() { buf.push((cur_len, cur_group)); }
        buf
    }).map(|groups| {
        let mut max = 0;

        for &(ref len, ref pairs) in groups.iter() {
            let mut nums = Vec::new();

            let start = num::pow(10u, *len) - 1;
            let end   = num::pow(10u, *len - 1);
            for n in range(arith::isqrt(end), arith::isqrt(start)).rev() {
                let ds = numconv::to_digits(n * n, 10).rev().collect::<Vec<uint>>();
                for &(ref v1, ref v2) in pairs.iter() {
                    if ds[v2[0]] == 0 { continue }
                    if !check_digit(v1.as_slice(), ds.as_slice()) { continue }
                    let num2 = idx_to_num(v2.as_slice(), ds.as_slice());
                    if !is_square(num2) { continue }
                    nums.push(n * n);
                    if n * n != num2 { nums.push(num2); }
                }
            }

            if !nums.is_empty() {
                max = nums.into_iter().max().unwrap();
                break
            }
        }
        max
    });

    match result {
        Err(msg) => panic!(msg),
        Ok(value) => value.to_string()
    }
}
