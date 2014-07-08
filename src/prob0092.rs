#![crate_name = "prob0092"]
#![crate_type = "rlib"]

extern crate num;

use num::Integer;

pub static EXPECTED_ANSWER: &'static str = "8581146";

fn square_digit_sum(mut n: uint) -> uint {
    let mut sum = 0;
    while n > 0 {
        let (d, m) = n.div_rem(&10);
        sum += m * m;
        n = d;
    }
    sum
}

fn is_reach_89(n: uint, map: &mut [Option<bool>]) -> bool {
    if n >= map.len() {
        return is_reach_89(square_digit_sum(n), map);
    }

    match map[n] {
        Some(b) => { return b; }
        None => { }
    }

    let result = is_reach_89(square_digit_sum(n), map);
    map[n] = Some(result);
    result
}

pub fn solve() -> String {
    let limit = 10000000;
    let mut cnt = 0u;

    let vec_size = 81 * 7 + 1;
    let mut map = Vec::from_elem(vec_size, None);
    *map.get_mut(0) = Some(false);
    *map.get_mut(1) = Some(false);
    *map.get_mut(89) = Some(true);
    for n in range(1u, limit + 1) {
        if is_reach_89(n, map.as_mut_slice()) { cnt += 1; }
    }
    cnt.to_str()
}
