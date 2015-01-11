//! [Problem 83](https://projecteuler.net/problem=83) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#[macro_use(problem)] extern crate common;

use std::{cmp, iter, uint};
use std::collections::HashSet;
use std::io::{BufferedReader, File, IoResult};

fn read_matrix<T: Reader>(reader: T) -> IoResult<Vec<Vec<uint>>> {
    let mut br = BufferedReader::new(reader);

    let mut mat = vec![];

    for line in br.lines() {
        let row = try!(line).trim().split(',').filter_map(StrExt::parse::<uint>).collect();
        mat.push(row);
    }

    Ok(mat)
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Point { x: uint, y: uint }

fn minimal_path_sum(mat: Vec<Vec<uint>>) -> uint {
    let (w, h) = (mat[0].len(), mat.len());

    let start = Point { x: 0,     y: 0 };
    let goal  = Point { x: w - 1, y: h - 1 };

    let mut closed = HashSet::new();
    let mut open   = HashSet::new();
    let mut dist   = range(0, h).map(|_| {
        iter::repeat(uint::MAX).take(w).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    let mut parent = range(0, h).map(|_| {
        iter::repeat(Point { x: w, y: h }).take(w).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    dist[start.y][start.x] = mat[start.y][start.x];
    open.insert(start);

    loop {
        if open.is_empty() { panic!(); }

        let &min_pt = open.iter()
            .min_by(|&pt| dist[pt.y][pt.x] + (h - pt.y - 1) + (w - pt.x - 1))
            .unwrap();

        if min_pt == goal { break }
        open.remove(&min_pt);
        closed.insert(min_pt);

        let mut ms = Vec::new();
        if min_pt.x > 0 { ms.push(Point { x: min_pt.x - 1, .. min_pt }) }
        if min_pt.y > 0 { ms.push(Point { y: min_pt.y - 1, .. min_pt }) }
        if min_pt.x < w - 1 { ms.push(Point { x: min_pt.x + 1, .. min_pt }) }
        if min_pt.y < h - 1 { ms.push(Point { y: min_pt.y + 1, .. min_pt }) }

        for &pt in ms.iter() {
            let new_dist = dist[min_pt.y][min_pt.x] + mat[pt.y][pt.x];
            if open.contains(&pt) {
                if new_dist < dist[pt.y][pt.x] {
                    dist[pt.y][pt.x]   = new_dist;
                    parent[pt.y][pt.x] = min_pt;
                }
                continue
            }
            if closed.contains(&pt) {
                if new_dist < dist[pt.y][pt.x] {
                    closed.remove(&pt);
                    dist[pt.y][pt.x]   = cmp::min(dist[pt.y][pt.x], new_dist);
                    parent[pt.y][pt.x] = min_pt;
                    open.insert(pt);
                }
                continue
            }
            dist[pt.y][pt.x]   = cmp::min(dist[pt.y][pt.x], new_dist);
            parent[pt.y][pt.x] = min_pt;
            open.insert(pt);
        }
    }

    dist[h - 1][w - 1]
}

fn solve(file: File) -> IoResult<String> {
    let mat = try!(read_matrix(file));
    Ok(minimal_path_sum(mat).to_string())
}

problem!("425185", "p083_matrix.txt", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn five_by_five() {
        let mat = vec![
            vec![131, 673, 234, 103,  18],
            vec![201,  96, 342, 965, 150],
            vec![630, 803, 746, 422, 111],
            vec![537, 699, 497, 121, 956],
            vec![805, 732, 524,  37, 331]
        ];
        assert_eq!(2297, super::minimal_path_sum(mat));
    }
}
