//! [Problem 107](https://projecteuler.net/problem=107) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
};
use union_find::{QuickUnionUf as Uf, UnionBySize, UnionFind};

fn compute<R: Read>(r: R, size: usize) -> io::Result<usize> {
    let mut verts = Vec::new();
    for (i, line) in BufReader::new(r).lines().enumerate() {
        let line = line?;
        for (j, s) in line.trim().split(',').enumerate() {
            if j <= i {
                continue;
            }
            if let Ok(n) = s.parse::<usize>() {
                verts.push(((i, j), n));
            }
        }
    }
    verts.sort_by(|a, b| a.1.cmp(&b.1));

    let mut uf = Uf::<UnionBySize>::new(size);

    let mut saving = 0;
    for &((i, j), n) in &verts {
        if uf.find(i) == uf.find(j) {
            saving += n;
        } else {
            let _ = uf.union(i, j);
        }
    }

    Ok(saving)
}

fn solve(file: File) -> io::Result<String> {
    Ok(compute(file, 40)?.to_string())
}

common::problem!("259679", "p107_network.txt", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn sample() {
        let matrix = r"-,16,12,21,-,-,-
16,-,-,17,20,-,-
12,-,-,28,-,31,-
21,17,28,-,18,19,23
-,20,-,18,-,-,11
-,-,31,19,-,-,27
-,-,-,23,11,27,-";

        assert_eq!(150, super::compute(matrix.as_bytes(), 7).unwrap());
    }
}
