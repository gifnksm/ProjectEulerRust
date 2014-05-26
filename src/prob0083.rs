#![crate_id = "prob0083"]
#![crate_type = "rlib"]

extern crate collections;
extern crate prob0081;

use std::{cmp, uint};
use collections::HashSet;

pub static EXPECTED_ANSWER: &'static str = "425185";

#[deriving(TotalEq, Eq, Hash, Clone)]
struct Point { x: uint, y: uint }

pub fn solve() -> String {
    let (w, h, mat) = prob0081::read_matrix("files/matrix.txt");
    let start = Point { x: 0,     y: 0 };
    let goal  = Point { x: w - 1, y: h - 1 };

    let mut closed = HashSet::new();
    let mut open   = HashSet::new();
    let mut dist = Vec::from_fn(h, |_y| Vec::from_elem(w, uint::MAX));
    let mut parent = Vec::from_fn(h, |_y| Vec::from_elem(w, Point { x: w, y: h }));

    *dist.get_mut(start.y).get_mut(start.x) = *mat.get(start.y).get(start.x);
    open.insert(start);

    loop {
        if open.is_empty() { fail!(); }

        let &min_pt = open.iter()
            .min_by(|&pt| *dist.get(pt.y).get(pt.x) + (h - pt.y - 1) + (w - pt.x - 1))
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
            let new_dist = *dist.get(min_pt.y).get(min_pt.x) + *mat.get(pt.y).get(pt.x);
            if open.contains(&pt) {
                if new_dist < *dist.get(pt.y).get(pt.x) {
                    *dist.get_mut(pt.y).get_mut(pt.x)   = new_dist;
                    *parent.get_mut(pt.y).get_mut(pt.x) = min_pt;
                }
                continue
            }
            if closed.contains(&pt) {
                if new_dist < *dist.get(pt.y).get(pt.x) {
                    closed.remove(&pt);
                    *dist.get_mut(pt.y).get_mut(pt.x) = cmp::min(*dist.get(pt.y).get(pt.x), new_dist);
                    *parent.get_mut(pt.y).get_mut(pt.x) = min_pt;
                    open.insert(pt);
                }
                continue
            }
            *dist.get_mut(pt.y).get_mut(pt.x) = cmp::min(*dist.get(pt.y).get(pt.x), new_dist);
            *parent.get_mut(pt.y).get_mut(pt.x) = min_pt;
            open.insert(pt);
        }
    }

    dist.get(h - 1).get(w - 1).to_str()
}
