#![crate_name = "prob0062"]
#![crate_type = "rlib"]

extern crate math;

use std::collections::{HashMap, HashSet};
use math::numconv;

pub const EXPECTED_ANSWER: &'static str = "127035954683";

pub fn solve() -> String {
    let mut map = HashMap::<Vec<uint>, Vec<uint>>::new();
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
        let mut ds = numconv::to_digits(cube, 10).collect::<Vec<uint>>();
        ds.sort();

        let v = match map.pop(&ds) {
            Some(mut nums) => { nums.push(cube); nums },
            None => vec!(cube)
        };
        if v.len() == 5 {
            set.insert(v[0]);
        }
        if v.len() == 6 {
            set.remove(&v[0]);
        }
        map.insert(ds, v);
    }

    return set.iter().min().unwrap().to_string();
}

