#[link(name = "prob0082", vers = "0.0", package_id = "prob0082")];
#[crate_type = "lib"];

extern mod prob0081;

use std::{uint, vec};

pub static EXPECTED_ANSWER: &'static str = "260324";

pub fn solve() -> ~str {
    let (w, h, mat) = prob0081::read_matrix("files/matrix.txt");

    let mut sum = vec::from_fn(h, |_y| vec::from_elem(w, 0u));
    for y in range(0, h) { sum[y][0] = mat[y][0]; }
    for x in range(1, w) {
        for y in range(0, h) {
            let mut min = sum[y][x - 1];

            let mut s = 0;
            for dy in range(1, y) {
                s += mat[y - dy][x];
                min = uint::min(sum[y - dy][x - 1] + s, min);
            }

            let mut s = 0;
            for dy in range(1, h - y) {
                s += mat[y + dy][x];
                min = uint::min(sum[y + dy][x - 1] + s, min);
            }

            sum[y][x] = mat[y][x] + min;
        }
    }
    let mut min = uint::max_value;
    for y in range(0, h) {
        min = uint::min(sum[y][w - 1], min);
    }
    min.to_str()
}
