#![crate_id = "prob0081"]
#![crate_type = "rlib"]

use std::cmp;
use std::io::{BufferedReader, File};

pub static EXPECTED_ANSWER: &'static str = "427337";

pub fn read_matrix(filename: &str) -> (uint, uint, ~[~[uint]]) {
    let mut br = BufferedReader::new(File::open(&Path::new(filename)).ok().expect("file not found."));

    let mut mat = Vec::new();
    for line in br.lines().filter_map(|line| line.ok()) {
        let row: ~[uint] = line.trim().split(',').filter_map(from_str::<uint>).collect();
        mat.push(row);
        assert_eq!(mat.get(0).len(), mat.last().unwrap().len());
    }
    (mat.get(0).len(), mat.len(), mat.move_iter().collect())
}

pub fn solve() -> ~str {
    let (w, h, mat) = read_matrix("files/matrix.txt");

    let mut sum: ~[~[uint]] = Vec::from_fn(h, |_y| {
        Vec::from_elem(w, 0u).move_iter().collect()
    }).move_iter().collect();

    sum[0][0] = mat[0][0];
    for y in range(1, h) {
        sum[y][0] = mat[y][0] + sum[y - 1][0];
    }
    for x in range(1, w) {
        sum[0][x] = mat[0][x] + sum[0][x - 1];
        for y in range(1, h) {
            sum[y][x] = mat[y][x] + cmp::min(sum[y - 1][x], sum[y][x - 1]);
        }
    }
    sum[h - 1][w - 1].to_str()
}
