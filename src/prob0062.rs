#[link(name = "prob0062", vers = "0.0")];
#[crate_type = "lib"];

extern mod extra;
extern mod math;

use std::vec;
use std::hashmap::{HashMap, HashSet};
use extra::sort;
use math::numconv;

pub static EXPECTED_ANSWER: &'static str = "127035954683";

pub fn solve() -> ~str {
    let mut map = HashMap::<~[uint], ~[uint]>::new();
    let mut set = HashSet::<uint>::new();
    let mut n     = 0;
    let mut limit = 10;
    loop {
        n += 1;
        if n >= limit {
            if !set.is_empty() {
                break;
            }
            limit *= 10;
        }

        let cube = n * n * n;
        let mut ds = numconv::to_digits(cube, 10).to_owned_vec();
        sort::quick_sort3(ds);

        let v = match map.pop(&ds) {
            Some(nums) => vec::append_one(nums, cube),
            None       => ~[cube]
        };
        if v.len() == 5 {
            set.insert(v[0]);
        }
        if v.len() == 6 {
            set.remove(&v[0]);
        }
        map.insert(ds, v);
    }

    return set.iter().min().unwrap().to_str();
}

