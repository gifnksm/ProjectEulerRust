#[crate_id = "prob0014"];
#[crate_type = "rlib"];

extern crate num;

use std::hashmap::HashMap;
use num::Integer;

pub static EXPECTED_ANSWER: &'static str = "837799";

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
    x
}

pub fn solve() -> ~str {
    let mut map = HashMap::new();
    map.insert(1u, 1u);

    range(2u, 1000000)
        .max_by(|&n| get_len(&mut map, n))
        .unwrap()
        .to_str()
}
