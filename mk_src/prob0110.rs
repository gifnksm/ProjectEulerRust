#![crate_name = "prob0110"]
#![crate_type = "rlib"]

extern crate math;
use std::collections::priority_queue::PriorityQueue;
use math::prime::Prime;

pub const EXPECTED_ANSWER: &'static str = "9350130049860600";

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
    fn partial_cmp(&self, other: &Elem) -> Option<Ordering> { Some(self.cmp(other)) }
}
impl Ord for Elem {
    fn cmp(&self, other: &Elem) -> Ordering {
        let &Elem(s, _) = self;
        let &Elem(o, _) = other;
        s.cmp(&o).reverse()
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
            return n.to_string();
        }
        if pairs.len() == 1 || pairs[pairs.len() - 1] < pairs[pairs.len() - 2] {
            let mut new_pairs = pairs.clone();
            new_pairs[pairs.len() - 1] += 1;
            queue.push(Elem(n * prime.nth(pairs.len() - 1), new_pairs));
        }
        pairs.push(1);
        queue.push(Elem(n * prime.nth(pairs.len() - 1), pairs));
    }
}
