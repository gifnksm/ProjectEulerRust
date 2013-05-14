#[link(name = "prob0079", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use core::hashmap::{ HashMap, HashSet };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 79,
    answer: "73162890",
    solver: solve
};

struct Relation<T> {
    num_prec: uint,
    succ: HashSet<T>
}

impl<T: Hash + IterBytes + Eq> Relation<T> {
    fn new() -> Relation<T> { Relation { num_prec: 0, succ: HashSet::new() } }
}

struct Relations<T> {
    top: HashMap<T, Relation<T>>
}

impl<T: Hash + IterBytes + Eq + Copy> Relations<T> {
    fn new() -> Relations<T> { Relations { top: HashMap::new() } }

    fn set_dependant(&mut self, prec: T, succ: T) {
        if !self.top.contains_key(&prec) {
            self.top.insert(prec, Relation::new());
        }
        if !self.top.contains_key(&succ) {
            self.top.insert(succ, Relation::new());
        }

        let mut contained = true;
        match self.top.find_mut(&prec) {
            Some(s) => {
                if !s.succ.contains(&succ) {
                    s.succ.insert(succ);
                    contained = false;
                }
            }
            None => { fail!() }
        }
        if !contained {
            match self.top.find_mut(&succ) {
                Some(p) => { p.num_prec += 1; }
                None => { fail!(); }
            }
        }
    }

    fn find_all_not_preceded(&self) -> ~[T] {
        let mut result = ~[];
        for self.top.each |k, v| {
            if v.num_prec == 0 { result.push(*k); }
        }
        return result;
    }

    fn delete_and_find(&mut self, prec: T) -> ~[T] {
        let mut result = ~[];
        do self.top.pop(&prec).map |p| {
            for p.succ.each |&s| {
                do self.top.find_mut(&s).map |&y| {
                    y.num_prec -= 1;
                    if y.num_prec == 0 {
                        result.push(s);
                    }
                };
            }
        };
        return result;
    }
}

fn tsort<T: Hash + IterBytes + Eq + Copy>(rels: &mut Relations<T>) -> ~[T] {
    let mut sorted = ~[];
    let mut queue = rels.find_all_not_preceded();
    while !queue.is_empty() {
        let prec = queue.shift();
        sorted.push(prec);
        queue.push_all(rels.delete_and_find(prec));
    }
    return sorted;
}


pub fn solve() -> ~str {
    let result = io::file_reader(&Path("files/keylog.txt")).map(|file| {
        let mut rels = Relations::new();
        for file.each_line |line| {
            let ds = vec::filter_map(str::to_chars(line), |c| char::to_digit(c, 10));
            for uint::range(1, ds.len()) |i| {
                rels.set_dependant(ds[i - 1], ds[i]);
            }
        }
        tsort(&mut rels)
    });

    match result {
        Err(msg) => fail!(msg),
        Ok(value) => return str::concat(value.map(|d| d.to_str()))
    }
}
