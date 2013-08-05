#[link(name = "prob0095", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::{vec, uint};
use common::extiter::Range;

pub static EXPECTED_ANSWER: &'static str = "14316";

#[inline(always)]
fn get_chain_len(mut n: uint, len_map: &mut [Option<uint>], div_map: &[uint]) -> uint {
    match len_map[n] {
        Some(x) => { return x; }
        None => {}
    }

    let mut itr_map = ~[n];
    loop {
        n = div_map[n];

        if n >= len_map.len() {
            for &n in itr_map.iter() { len_map[n] = Some(0); }
            return 0;
        }

        match itr_map.position_elem(&n) {
            Some(idx) => {
                let len = itr_map.len() - idx;
                for (i, &m) in itr_map.iter().enumerate() {
                    len_map[m] = Some(if i < idx { 0 } else { len });
                }
                return if idx != 0 { 0 } else { len };
            }
            None => { itr_map.push(n); }
        }
    }
}

pub fn solve() -> ~str {
    let limit = 1000000;
    let mut len_map = vec::from_elem(limit + 1, None);
    let mut div_map = vec::from_elem(limit + 1, 1u);
    div_map[0] = 0;
    div_map[1] = 0;

    for f in range(2, limit / 2) {
        do uint::range_step(2 * f, limit, f as int) |n| {
            div_map[n] += f;
            true
        };
    }

    let (n, _div) = Range::new(1, len_map.len())
        .transform(|n| (n, get_chain_len(n, len_map, div_map)))
        .max_by(|&(_n, div)| div)
        .unwrap();

    return n.to_str();
}
