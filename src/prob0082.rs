#![crate_name = "prob0082"]
#![crate_type = "rlib"]

extern crate prob0081;

use std::{cmp, uint};

pub static EXPECTED_ANSWER: &'static str = "260324";

pub fn solve() -> String {
    let (w, h, mat) = prob0081::read_matrix("files/matrix.txt");

    let mut sum = Vec::from_fn(h, |_y| Vec::from_elem(w, 0u));

    for y in range(0, h) { *sum.get_mut(y).get_mut(0) = *mat.get(y).get(0); }
    for x in range(1, w) {
        for y in range(0, h) {
            let mut min = *sum.get(y).get(x - 1);

            let mut s = 0;
            for dy in range(1, y) {
                s += *mat.get(y - dy).get(x);
                min = cmp::min(*sum.get(y - dy).get(x - 1) + s, min);
            }

            let mut s = 0;
            for dy in range(1, h - y) {
                s += *mat.get(y + dy).get(x);
                min = cmp::min(*sum.get(y + dy).get(x - 1) + s, min);
            }

            *sum.get_mut(y).get_mut(x) = mat.get(y).get(x) + min;
        }
    }
    let mut min = uint::MAX;
    for y in range(0, h) {
        min = cmp::min(*sum.get(y).get(w - 1), min);
    }
    min.to_str()
}
