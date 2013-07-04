#[link(name = "prob0043", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::{uint, vec};
use std::iterator::AdditiveIterator;
use common::calc;

pub static EXPECTED_ANSWER: &'static str = "16695334890";

struct DigitMap { used: [ bool, ..10] }

fn DigitMap() -> DigitMap {
    DigitMap { used: [ false, ..10 ] }
}

impl DigitMap {
    fn is_used(&self, n: uint) -> bool {
        assert!(n < 10);
        self.used[n]
    }

    priv fn get_used(&self, ds: &[uint]) -> Option<DigitMap> {
        let mut used: [bool, ..10] = [false, ..10];
        for ds.iter().advance |&d| {
            assert!(d < 10);
            if used[d] || self.is_used(d) { return None; }
            used[d] = true;
        }

        Some(DigitMap { used: [
            self.used[0] || used[0],
            self.used[1] || used[1],
            self.used[2] || used[2],
            self.used[3] || used[3],
            self.used[4] || used[4],
            self.used[5] || used[5],
            self.used[6] || used[6],
            self.used[7] || used[7],
            self.used[8] || used[8],
            self.used[9] || used[9]
        ] })
    }
}

fn fill_vec<T: Copy>(v: ~[T], len: uint, init: T) -> ~[T] {
    assert!(v.len() <= len);
    if v.len() == len { return v; }
    return vec::from_elem(len - v.len(), init) + v;
}

pub fn solve() -> ~str {
    let mut result: ~[(~[uint], DigitMap)] = ~[(~[], DigitMap())];
    result = do result.flat_map |tp| {
        let mut arr = ~[];
        let dm = tp.second_ref();
        for uint::range(0, 999 / 17) |n| {
            let ds = fill_vec(calc::num_to_digits(n * 17, 10), 3, 0);
            match dm.get_used(ds) {
                None => loop,
                Some(e) => arr.push((ds + *tp.first_ref(), e))
            }
        }
        arr
    };
    let base_list = [13u, 11, 7, 5, 3, 2, 1];
    for base_list.iter().advance |&base| {
        result = do result.flat_map |tp| {
            let mut arr = ~[];
            let dm = tp.second_ref();
            for uint::range(0, 999 / base) |n| {
                let ds = fill_vec(calc::num_to_digits(n * base, 10), 3, 0);
                if ds[1] != tp.first_ref()[0] || ds[2] != tp.first_ref()[1] {
                    loop
                }
                match dm.get_used([ds[0]]) {
                    None => loop,
                    Some(e) => arr.push((~[ds[0]] + *tp.first_ref(), e))
                }
            }
            arr
        };
    }

    return result.iter()
        .transform(|&r| calc::digits_to_num(r.first(), 10))
        .sum()
        .to_str();
}
