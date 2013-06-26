#[link(name = "prob0062", vers = "0.0")];
#[crate_type = "lib"];

extern mod extra;
extern mod common;

use std::hashmap::{HashMap, HashSet};
use extra::sort;
use common::calc;
use common::problem::Problem;

pub static problem: Problem<'static> = Problem {
    id: 62,
    answer: "127035954683",
    solver: solve
};

pub fn solve() -> ~str {
    let mut map = HashMap::new::<~[uint], ~[uint]>();
    let mut set = HashSet::new::<uint>();
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
        let mut ds = calc::num_to_digits(cube, 10);
        sort::quick_sort3(ds);

        let v = match map.pop(&ds) {
            Some(nums) => nums + [ cube ],
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

