#[crate_type = "rlib"];

extern mod data;
extern mod common;

use std::io::buffered::BufferedReader;
use std::io::File;
use data::union_find::UnionFind;

pub static EXPECTED_ANSWER: &'static str = "259679";

pub fn solve() -> ~str {
    let size = 40;
    let mut br = BufferedReader::new(
        File::open(&Path::new("files/network.txt")).expect("file not found."));

    let mut verts = ~[];
    for (i, line) in br.lines().enumerate() {
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
