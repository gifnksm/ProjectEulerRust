#[crate_type = "rlib"];

extern mod prob0081;

use std::{cmp, vec, uint};
use std::hashmap::HashSet;

pub static EXPECTED_ANSWER: &'static str = "425185";

#[deriving(Eq, IterBytes, Clone)]
struct Point { x: uint, y: uint }

pub fn solve() -> ~str {
    let (w, h, mat) = prob0081::read_matrix("files/matrix.txt");
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

        let &min_pt = open.iter()
            .min_by(|&pt| dist[pt.y][pt.x] + (h - pt.y - 1) + (w - pt.x - 1))
            .unwrap();

        if min_pt == goal { break }
        open.remove(&min_pt);
        closed.insert(min_pt);

        let mut ms = ~[];
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
                    dist[pt.y][pt.x] = cmp::min(dist[pt.y][pt.x], new_dist);
                    parent[pt.y][pt.x] = min_pt;
                    open.insert(pt);
                }
                continue
            }
            dist[pt.y][pt.x] = cmp::min(dist[pt.y][pt.x], new_dist);
            parent[pt.y][pt.x] = min_pt;
            open.insert(pt);
        }
    }

    dist[h - 1][w - 1].to_str()
}
