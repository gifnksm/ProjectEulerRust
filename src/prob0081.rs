#[link(name = "prob0081", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::{uint, vec};
use std::rt::io;
use std::rt::io::buffered::BufferedReader;
use std::rt::io::file::FileInfo;
use common::reader::BufferedReaderUtil;

pub static EXPECTED_ANSWER: &'static str = "427337";

pub fn read_matrix(filename: &str) -> (uint, uint, ~[~[uint]]) {
    let r = Path::new(filename).open_reader(io::Open).expect("file not found.");
    let mut br = BufferedReader::new(r);

    let mut mat = ~[];
    for line in br.line_iter() {
        mat.push(line.trim().split_iter(',').filter_map(from_str::<uint>).to_owned_vec());
        assert_eq!(mat[0].len(), mat.last().len());
    }
    (mat[0].len(), mat.len(), mat)
}

pub fn solve() -> ~str {
    let (w, h, mat) = read_matrix("files/matrix.txt");

    let mut sum = vec::from_fn(h, |_y| vec::from_elem(w, 0u));
    sum[0][0] = mat[0][0];
    for y in range(1, h) {
        sum[y][0] = mat[y][0] + sum[y - 1][0];
    }
    for x in range(1, w) {
        sum[0][x] = mat[0][x] + sum[0][x - 1];
        for y in range(1, h) {
            sum[y][x] = mat[y][x] + uint::min(sum[y - 1][x], sum[y][x - 1]);
        }
    }
    sum[h - 1][w - 1].to_str()
}
