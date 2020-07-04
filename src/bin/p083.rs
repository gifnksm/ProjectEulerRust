//! [Problem 83](https://projecteuler.net/problem=83) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use std::{
    cmp,
    collections::HashSet,
    fs::File,
    io::{self, prelude::*, BufReader},
    u32,
};

fn read_matrix<T: Read>(reader: T) -> io::Result<Vec<Vec<u32>>> {
    let mut mat = vec![];

    for line in BufReader::new(reader).lines() {
        let row = line?
            .trim()
            .split(',')
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();
        mat.push(row);
    }

    Ok(mat)
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

fn minimal_path_sum(mat: Vec<Vec<u32>>) -> u32 {
    let (w, h) = (mat[0].len(), mat.len());

    let start = Point { x: 0, y: 0 };
    let goal = Point { x: w - 1, y: h - 1 };

    let mut closed = HashSet::new();
    let mut open = HashSet::new();
    let mut dist = vec![vec![u32::MAX; w]; h];
    let mut parent = vec![vec![Point { x: w, y: h }; w]; h];

    dist[start.y][start.x] = mat[start.y][start.x];
    let _ = open.insert(start);

    loop {
        if open.is_empty() {
            panic!();
        }

        let &min_pt = open
            .iter()
            .min_by_key(|&pt| dist[pt.y][pt.x] + ((h - pt.y - 1) + (w - pt.x - 1)) as u32)
            .unwrap();

        if min_pt == goal {
            break;
        }
        let _ = open.remove(&min_pt);
        let _ = closed.insert(min_pt);

        let mut ms = Vec::new();
        if min_pt.x > 0 {
            ms.push(Point {
                x: min_pt.x - 1,
                ..min_pt
            })
        }
        if min_pt.y > 0 {
            ms.push(Point {
                y: min_pt.y - 1,
                ..min_pt
            })
        }
        if min_pt.x < w - 1 {
            ms.push(Point {
                x: min_pt.x + 1,
                ..min_pt
            })
        }
        if min_pt.y < h - 1 {
            ms.push(Point {
                y: min_pt.y + 1,
                ..min_pt
            })
        }

        for &pt in &ms {
            let new_dist = dist[min_pt.y][min_pt.x] + mat[pt.y][pt.x];
            if open.contains(&pt) {
                if new_dist < dist[pt.y][pt.x] {
                    dist[pt.y][pt.x] = new_dist;
                    parent[pt.y][pt.x] = min_pt;
                }
                continue;
            }
            if closed.contains(&pt) {
                if new_dist < dist[pt.y][pt.x] {
                    let _ = closed.remove(&pt);
                    dist[pt.y][pt.x] = cmp::min(dist[pt.y][pt.x], new_dist);
                    parent[pt.y][pt.x] = min_pt;
                    let _ = open.insert(pt);
                }
                continue;
            }
            dist[pt.y][pt.x] = cmp::min(dist[pt.y][pt.x], new_dist);
            parent[pt.y][pt.x] = min_pt;
            let _ = open.insert(pt);
        }
    }

    dist[h - 1][w - 1]
}

fn solve(file: File) -> io::Result<String> {
    let mat = read_matrix(file)?;
    Ok(minimal_path_sum(mat).to_string())
}

common::problem!("425185", "p083_matrix.txt", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn five_by_five() {
        let mat = vec![
            vec![131, 673, 234, 103, 18],
            vec![201, 96, 342, 965, 150],
            vec![630, 803, 746, 422, 111],
            vec![537, 699, 497, 121, 956],
            vec![805, 732, 524, 37, 331],
        ];
        assert_eq!(2297, super::minimal_path_sum(mat));
    }
}
