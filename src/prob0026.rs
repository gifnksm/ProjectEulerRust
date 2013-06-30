#[link(name = "prob0026", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::vec;
use common::extiter::Range;

pub static expected_answer: &'static str = "983";

fn get_cycle_len(n: uint) -> uint {
    if n == 1 { return 1; }
    let mut buf = vec::from_elem(n, None);
    let mut rem = 1;
    let mut idx = 1;
    loop {
        let new_rem = rem % n;
        match buf[new_rem] {
            Some(i) => { return idx - i; }
            None    => { buf[new_rem] = Some(idx); }
        }
        idx += 1;
        rem = new_rem * 10;
    }
}

pub fn solve() -> ~str {
    return Range::new::<uint>(2, 1000)
        .max_by(|&n| get_cycle_len(n))
        .unwrap()
        .to_str();
}
