use core::hashmap::linear::{ LinearMap };

use std::sort::{ quick_sort3 };

use common::calc::{ num_to_digits };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 62,
    answer: "127035954683",
    solver: solve
};

fn solve() -> ~str {
    let mut map = LinearMap::new::<~[uint], ~[uint]>();
    let mut n = 0;
    loop {
        n += 1;
        let cube = n * n * n;
        let mut ds = num_to_digits(cube, 10);
        quick_sort3(ds);

        let v = match map.pop(&ds) {
            Some(nums) => nums + ~[ cube ],
            None       => ~[cube]
        };
        if v.len() == 5 {
            return v[0].to_str();
        }
        map.insert(ds, v);
    }
}

