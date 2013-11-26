#[link(name = "prob0098", vers = "0.0", package_id = "prob0098")];
#[crate_type = "lib"];

extern mod extra;
extern mod common;
extern mod math;

use std::{str, util, uint, vec};
use std::iter::OrdIterator;
use std::hashmap::HashMap;
use std::io::File;
use extra::sort;
use common::reader;
use math::{arith, numconv};

pub static EXPECTED_ANSWER: &'static str = "18769";

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

pub fn solve() -> ~str {
    let mut reader = File::open(&Path::new("files/words.txt")).expect("file not found.");
    let input = str::from_utf8_owned(reader.read_to_end());

    let result = (do reader::read_whole_word(input).map |words| {
        let mut map = ~HashMap::new();
        for &word in words.iter() {
            let mut cs = word.chars().to_owned_vec();
            sort::quick_sort(cs, |a, b| a <= b);
            match map.pop(&cs) {
                None     => { map.insert(cs, ~[word.to_str()]); }
                Some(ws) => { map.insert(cs, vec::append_one(ws, word.to_str())); }
            }
        }
        do vec::build(None) |push| {
            for (_key, values) in map.mut_iter() {
                if values.len() > 1 {
                    push(util::replace(values, ~[]));
                }
            }
        }
    }).map(|words| {
            do vec::build(Some(words.len())) |push| {
                for elt in words.iter() {
                    for i in range(0, elt.len()) {
                        for j in range(i + 1, elt.len()) {
                            push((elt[i].clone(), elt[j].clone()))
                        }
                    }
                }
            }
        }).map(|word_pairs| {
            let mut words = do word_pairs.map |&(ref w1, ref w2)| {
                let cs1 = w1.as_bytes();
                let cs2 = w2.as_bytes();
                let get_pos = |&c: &u8| cs1.position_elem(&c).unwrap();
                (w1.len(), cs1.map(|c| get_pos(c)), cs2.map(|c| get_pos(c)))
            };
            sort::quick_sort(words, |&(l1, _, _), &(l2, _, _)| l1 >= l2);
            words
        }).map(|idx_pairs| {
            do vec::build(None) |push| {
                let mut cur_len = uint::max_value;
                let mut cur_group = ~[];
                for &(ref len, ref v1, ref v2) in idx_pairs.iter() {
                    if cur_group.is_empty() || cur_len == *len {
                        cur_len = *len;
                        cur_group.push((v1.clone(), v2.clone()));
                    } else {
                        push((cur_len, util::replace(&mut cur_group, ~[(v1.clone(), v2.clone())])));
                        cur_len = *len;
                    }
                }
                if !cur_group.is_empty() { push((cur_len, cur_group)); }
            }
        }).map(|groups| {
            let mut max = 0;

            for &(ref len, ref pairs) in groups.iter() {
                let mut nums = ~[];

                let start = arith::pow(10, *len) - 1;
                let end   = arith::pow(10, *len - 1);
                for n in range(arith::isqrt(end), arith::isqrt(start)).invert() {
                    let ds = numconv::to_digits(n * n, 10).invert().to_owned_vec();
                    for &(ref v1, ref v2) in pairs.iter() {
                        if ds[v2[0]] == 0 { continue }
                        if !check_digit(*v1, ds) { continue }
                        let num2 = idx_to_num(*v2, ds);
                        if !is_square(num2) { continue }
                        nums.push(n * n);
                        if n * n != num2 { nums.push(num2); }
                    }
                }

                if !nums.is_empty() {
                    max = nums.move_iter().max().unwrap();
                    break
                }
            }
            max
        });

    match result {
        Err(msg) => fail!(msg),
        Ok(value) => value.to_str()
    }
}
