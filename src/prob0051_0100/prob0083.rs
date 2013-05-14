#[link(name = "prob0083", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use core::to_bytes::{ IterBytes };
use core::hashmap::{ HashSet };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 83,
    answer: "425185",
    solver: solve
};

#[deriving(Eq, IterBytes)]
struct Point { x: uint, y: uint }

pub fn solve() -> ~str {
    let result = io::file_reader(&Path("files/matrix.txt")).map(|file| {
        let mut mat = ~[];
        for file.each_line |line| {
            let mut row = ~[];
            for line.each_split_char(',') |n| {
                row.push(uint::from_str(n).get());
            }
            mat.push(row);
            assert_eq!(mat[0].len(), mat.last().len());
        }
        let w = mat[0].len();
        let h = mat.len();
        ((w, h), mat)
    }).map(|&((w, h), mat)| {
        let start = Point { x: 0,     y: 0 };
        let goal  = Point { x: w - 1, y: h - 1 };

        let mut closed = HashSet::new();
        let mut open   = HashSet::new();
        let mut dist = vec::from_fn(h, |_y| vec::from_elem(w, uint::max_value));
        let mut parent = vec::from_fn(h, |_y| vec::from_elem(w, Point { x: w, y: h }));

        dist[start.y][start.x] = mat[start.y][start.x];
        open.insert(start);

        loop {
            if open.is_empty() { fail!(); }

            let mut min_dist = uint::max_value;
            let mut min_pt   = Point { x: w, y: h};
            for open.each |&pt| {
                let d = dist[pt.y][pt.x] + (h - pt.y - 1) + (w - pt.x - 1);
                if d < min_dist {
                    min_dist = d;
                    min_pt = pt;
                }
            }

            if min_pt == goal { break; }
            open.remove(&min_pt);
            closed.insert(min_pt);

            let mut ms = ~[];
            if min_pt.x > 0 { ms.push(Point { x: min_pt.x - 1, .. min_pt }) }
            if min_pt.y > 0 { ms.push(Point { y: min_pt.y - 1, .. min_pt }) }
            if min_pt.x < w - 1 { ms.push(Point { x: min_pt.x + 1, .. min_pt }) }
            if min_pt.y < h - 1 { ms.push(Point { y: min_pt.y + 1, .. min_pt }) }

            for ms.each |&pt| {
                let new_dist = dist[min_pt.y][min_pt.x] + mat[pt.y][pt.x];
                if open.contains(&pt) {
                    if new_dist < dist[pt.y][pt.x] {
                        dist[pt.y][pt.x]   = new_dist;
                        parent[pt.y][pt.x] = min_pt;
                    }
                    loop;
                }
                if closed.contains(&pt) {
                    if new_dist < dist[pt.y][pt.x] {
                        closed.remove(&pt);
                        dist[pt.y][pt.x] = uint::min(dist[pt.y][pt.x], new_dist);
                        parent[pt.y][pt.x] = min_pt;
                        open.insert(pt);
                    }
                    loop;
                }
                dist[pt.y][pt.x] = uint::min(dist[pt.y][pt.x], new_dist);
                parent[pt.y][pt.x] = min_pt;
                open.insert(pt);
            }
        }

        dist[h - 1][w - 1]
    });

    match result {
        Err(msg) => fail!(msg),
        Ok(value) => return value.to_str()
    }
}
