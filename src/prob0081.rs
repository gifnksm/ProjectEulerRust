#[link(name = "prob0081", vers = "0.0")];
#[crate_type = "lib"];



use std::{uint, vec, io};

pub static EXPECTED_ANSWER: &'static str = "427337";

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
            sum[0][0] = mat[0][0];
            for y in range(1, *h) {
                sum[y][0] = mat[y][0] + sum[y - 1][0];
            }
            for x in range(1, *w) {
                sum[0][x] = mat[0][x] + sum[0][x - 1];
                for y in range(1, *h) {
                    sum[y][x] = mat[y][x] + uint::min(sum[y - 1][x], sum[y][x - 1]);
                }
            }
            sum[h - 1][w - 1]
        });

    match result {
        Err(msg) => fail!(msg),
        Ok(value) => return value.to_str()
    }
}
