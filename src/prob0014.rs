#[link(name = "prob0014", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::hashmap::HashMap;
use common::extiter::Range;
use common::problem::Problem;

pub static problem: Problem<'static> = Problem {
    id: 14,
    answer: "837799",
    solver: solve
};

fn get_len(map: &mut HashMap<uint, uint>, n: uint) -> uint {
    match map.find(&n) {
        Some(&x) => { return x; }
        None => {}
    }

    let x = if n.is_even() {
        get_len(map, n / 2) + 1
    } else {
        get_len(map, 3 * n + 1) + 1
    };
    map.insert(n, x);
    return x;
}

pub fn solve() -> ~str {
    let mut map = HashMap::new();
    map.insert(1u, 1u);

    return Range::new::<uint>(2, 1000000)
        .max_by(|&n| get_len(&mut map, n))
        .unwrap()
        .to_str();
}
