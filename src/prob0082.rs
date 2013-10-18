#[link(name = "prob0082", vers = "0.0")];
#[crate_type = "lib"];

use std::{uint, vec, io};

pub static EXPECTED_ANSWER: &'static str = "260324";

pub fn solve() -> ~str {
    let result = io::file_reader(&Path::new("files/matrix.txt"))
        .map(|file| {
            let mut mat = ~[];
            do file.each_line |line| {
                mat.push(line.split_iter(',').filter_map(from_str::<uint>).to_owned_vec());
                assert_eq!(mat[0].len(), mat.last().len());
                true
            };
            let w = mat[0].len();
            let h = mat.len();
            ((w, h), mat)
        }).map(|&((ref w, ref h), ref mat)| {
            let mut sum = vec::from_fn(*h, |_y| vec::from_elem(*w, 0u));
            for y in range(0, *h) { sum[y][0] = mat[y][0]; }
            for x in range(1, *w) {
                for y in range(0, *h) {
                    let mut min = sum[y][x - 1];

                    let mut s = 0;
                    for dy in range(1, y) {
                        s += mat[y - dy][x];
                        min = uint::min(sum[y - dy][x - 1] + s, min);
                    }

                    let mut s = 0;
                    for dy in range(1, *h - y) {
                        s += mat[y + dy][x];
                        min = uint::min(sum[y + dy][x - 1] + s, min);
                    }

                    sum[y][x] = mat[y][x] + min;
                }
            }
            let mut min = uint::max_value;
            for y in range(0, *h) {
                min = uint::min(sum[y][w - 1], min);
            }
            min
        });

    match result {
        Err(msg) => fail!(msg),
        Ok(value) => return value.to_str()
    }
}
