#![crate_id = "prob0107"]
#![crate_type = "rlib"]

extern crate data;

use std::io::{BufferedReader, File};
use data::union_find::UnionFind;

pub static EXPECTED_ANSWER: &'static str = "259679";

pub fn solve() -> ~str {
    let size = 40;
    let mut br = BufferedReader::new(
        File::open(&Path::new("files/network.txt")).ok().expect("file not found."));

    let mut verts = Vec::new();
    for (i, line) in br.lines().filter_map(|line| line.ok()).enumerate() {
        for (j, s) in line.trim().split(',').enumerate() {
            let n = from_str::<uint>(s);
            if i < j && n.is_some() {
                verts.push(((i, j), n.unwrap()));
            }
        }
    }
    verts.sort_by(|&(_, a), &(_, b)| a.cmp(&b));

    let mut uf = UnionFind::new(size);

    let mut saving = 0;
    for &((i, j), n) in verts.iter() {
        if uf.find(i, j) {
            saving += n;
        } else {
            uf.union(i, j);
        }
    }
    saving.to_str()
}
