#[link(name = "prob0081", vers = "0.0")];
#[crate_type = "lib"];



use std::{uint, vec, io};

pub static EXPECTED_ANSWER: &'static str = "427337";

pub fn solve() -> ~str {
    let result = io::file_reader(&Path("files/matrix.txt")).map(|file| {
        let mut mat = ~[];
        for file.each_line |line| {
            mat.push(line.split_iter(',').filter_map(uint::from_str).collect::<~[uint]>());
            assert_eq!(mat[0].len(), mat.last().len());
        }
        let w = mat[0].len();
        let h = mat.len();
        ((w, h), mat)
    }).map(|&((ref w, ref h), ref mat)| {
        let mut sum = vec::from_fn(*h, |_y| vec::from_elem(*w, 0u));
        sum[0][0] = mat[0][0];
        foreach y in range(1, *h) {
            sum[y][0] = mat[y][0] + sum[y - 1][0];
        }
        foreach x in range(1, *w) {
            sum[0][x] = mat[0][x] + sum[0][x - 1];
            foreach y in range(1, *h) {
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
