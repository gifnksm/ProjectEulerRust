#[link(name = "prob0107", vers = "0.0")];
#[crate_type = "lib"];

extern mod extra;
extern mod data;
extern mod common;

use std::io::buffered::BufferedReader;
use std::io::File;
use extra::sort;
use common::reader::BufferedReaderUtil;
use data::union_find::UnionFind;

pub static EXPECTED_ANSWER: &'static str = "259679";

pub fn solve() -> ~str {
    let size = 40;
    let mut br = BufferedReader::new(
        File::open(&Path::new("files/network.txt")).expect("file not found."));

    let mut verts = ~[];
    for (i, line) in br.line_iter().enumerate() {
        for (j, s) in line.trim().split_iter(',').enumerate() {
            let n = from_str::<uint>(s);
            if i < j && n.is_some() {
                verts.push(((i, j), n.unwrap()));
            }
        }
    }

    sort::quick_sort(verts, |&(_, a), &(_, b)| a.le(&b));
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
