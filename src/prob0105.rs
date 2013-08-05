#[link(name = "prob0105", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;
extern mod extra;

use std::{io, uint, vec};
use std::iterator::AdditiveIterator;
use extra::sort::Sort;
use common::reader::ReaderIterator;

pub static EXPECTED_ANSWER: &'static str = "73702";

fn is_sss(nums: &[uint]) -> bool {
    let mut sums: ~[uint] = ~[0];
    for &n in nums.iter() {
        let mut i = 0;
        let mut j = 0;
        let len = sums.len();
        let mut new_sums = vec::with_capacity(len * 2);
        while i < len {
            assert!(j <= i);
            match sums[i].cmp(&(sums[j] + n)) {
                Equal => { return false; }
                Less => {
                    new_sums.push(sums[i]);
                    i += 1;
                }
                Greater => {
                    new_sums.push(sums[j] + n);
                    j += 1;
                }
            }
        }

        while j < len {
            new_sums.push(sums[j] + n);
            j += 1;
        }

        sums = new_sums;
    }

    return true;
}

pub fn solve() -> ~str {
    let result = io::file_reader(&Path("files/sets.txt"))
        .map(|reader| {
            reader
                .line_iter()
                .transform(|line| {
                    line.split_iter(',')
                        .filter_map(uint::from_str)
                        .collect::<~[uint]>()
                }).transform(|mut nums| { nums.qsort(); nums })
                .filter(|nums| {
                    let len = nums.len();
                    let len_hd = (len + 1) / 2;
                    let len_tl = len_hd - 1;
                    let mut hd = nums.slice(0, len_hd).iter().transform(|&x| x);
                    let mut tl = nums.slice(len - len_tl, len).iter().transform(|&x| x);
                    hd.sum() > tl.sum()
                }).filter(|nums| is_sss(*nums))
                .transform(|nums| nums.iter().transform(|&x| x).sum())
                .sum()
        });

    match result {
        Err(msg) => fail!(msg),
        Ok(value) => return value.to_str()
    }
}
