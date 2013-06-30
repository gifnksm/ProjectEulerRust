#[link(name = "prob0079", vers = "0.0")];
#[crate_type = "lib"];



use std::{io, uint, char};
use std::hashmap::{HashMap, HashSet};

pub static expected_answer: &'static str = "73162890";

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
            self.top.insert(copy prec, Relation::new());
        }
        if !self.top.contains_key(&succ) {
            self.top.insert(copy succ, Relation::new());
        }

        let mut contained = true;
        match self.top.find_mut(&prec) {
            Some(s) => {
                if !s.succ.contains(&succ) {
                    s.succ.insert(copy succ);
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
        return self.top.iter()
            .filter(|&(&_k, &v)| v.num_prec == 0)
            .transform(|(&k, &_v)| k)
            .collect::<~[T]>();
    }

    fn delete_and_find(&mut self, prec: T) -> ~[T] {
        let mut result = ~[];
        do self.top.pop(&prec).map |p| {
            for p.succ.iter().advance |&s| {
                do self.top.find_mut(&s).map |&y| {
                    y.num_prec -= 1;
                    if y.num_prec == 0 {
                        result.push(copy s);
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
        sorted.push(copy prec);
        queue.push_all(rels.delete_and_find(prec));
    }
    return sorted;
}


pub fn solve() -> ~str {
    let result = io::file_reader(&Path("files/keylog.txt")).map(|file| {
        let mut rels = Relations::new();
        for file.each_line |line| {
            let ds: ~[uint] = line.iter().filter_map(|c| char::to_digit(c, 10)).collect();
            for uint::range(1, ds.len()) |i| {
                rels.set_dependant(ds[i - 1], ds[i]);
            }
        }
        tsort(&mut rels)
    });

    match result {
        Err(msg) => fail!(msg),
        Ok(value) => return value.map(|d| d.to_str()).concat()
    }
}
