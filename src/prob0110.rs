#![crate_id = "prob0110"]
#![crate_type = "rlib"]

extern crate math;
use std::collections::priority_queue::PriorityQueue;
use math::prime::Prime;

pub static EXPECTED_ANSWER: &'static str = "9350130049860600";

struct Elem (uint, Vec<uint>);

impl PartialEq for Elem {
    fn eq(&self, other: &Elem) -> bool {
        let &Elem(s, _) = self;
        let &Elem(o, _) = other;
        s == o
    }
}
impl Eq for Elem {}
impl PartialOrd for Elem {
    fn lt(&self, other: &Elem) -> bool {
        let &Elem(s, _) = self;
        let &Elem(o, _) = other;
        s.gt(&o)
    }
}
impl Ord for Elem {
    fn cmp(&self, other: &Elem) -> Ordering {
        let &Elem(s, _) = self;
        let &Elem(o, _) = other;
        match s.cmp(&o) {
            Less    => Greater,
            Equal   => Equal,
            Greater => Less
        }
    }
}


pub fn solve() -> String {
    let limit = 4000000;

    let prime = Prime::new();
    let mut queue = PriorityQueue::new();
    queue.push(Elem(2u, vec![1u]));

    loop {
        let Elem(n, mut pairs) = queue.pop().unwrap();
        let num_sol = (pairs.iter().fold(1, |n, &i| n * (2 * i + 1)) + 1) / 2;
        if num_sol > limit {
            return n.to_str();
        }
        if pairs.len() == 1 || *pairs.get(pairs.len() - 1) < *pairs.get(pairs.len() - 2) {
            let mut new_pairs = pairs.clone();
            *new_pairs.get_mut(pairs.len() - 1) += 1;
            queue.push(Elem(n * prime.nth(pairs.len() - 1), new_pairs));
        }
        pairs.push(1);
        queue.push(Elem(n * prime.nth(pairs.len() - 1), pairs));
    }
}
