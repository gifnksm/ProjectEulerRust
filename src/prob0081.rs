#![crate_id = "prob0081"]
#![crate_type = "rlib"]

use std::cmp;
use std::io::{BufferedReader, File};

pub static EXPECTED_ANSWER: &'static str = "427337";

pub fn read_matrix(filename: &str) -> (uint, uint, Vec<Vec<uint>>) {
    let mut br = BufferedReader::new(File::open(&Path::new(filename)).ok().expect("file not found."));

    let mut mat: Vec<Vec<uint>> = Vec::new();
    for line in br.lines().filter_map(|line| line.ok()) {
        let row = line.trim().split(',').filter_map(from_str::<uint>).collect();
        mat.push(row);
        assert_eq!(mat.get(0).len(), mat.last().unwrap().len());
    }
    (mat.get(0).len(), mat.len(), mat)
}

pub fn solve() -> ~str {
    let (w, h, mat) = read_matrix("files/matrix.txt");

    let mut sum = Vec::from_fn(h, |_y| Vec::from_elem(w, 0u));

    *sum.get_mut(0).get_mut(0) = *mat.get(0).get(0);
    for y in range(1, h) {
        *sum.get_mut(y).get_mut(0) = *mat.get(y).get(0) + *sum.get(y - 1).get(0);
    }
    for x in range(1, w) {
        *sum.get_mut(0).get_mut(x) = *mat.get(0).get(x) + *sum.get(0).get(x - 1);
        for y in range(1, h) {
            *sum.get_mut(y).get_mut(x) = *mat.get(y).get(x)
                + cmp::min(*sum.get(y - 1).get(x), *sum.get(y).get(x - 1));
        }
    }
    sum.get(h - 1).get(w - 1).to_str()
}
