#[link(name = "prob0079", vers = "0.0")];
#[crate_type = "lib"];

use std::{io, char};
use std::hashmap::{HashMap, HashSet};

pub static EXPECTED_ANSWER: &'static str = "73162890";

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

impl<T: Hash + IterBytes + Eq + Clone> Relations<T> {
    fn new() -> Relations<T> { Relations { top: HashMap::new() } }

    fn set_dependant(&mut self, prec: T, succ: T) {
        if !self.top.contains_key(&prec) {
            self.top.insert(prec.clone(), Relation::new());
        }
        if !self.top.contains_key(&succ) {
            self.top.insert(succ.clone(), Relation::new());
        }

        let mut contained = true;
        match self.top.find_mut(&prec) {
            Some(s) => {
                if !s.succ.contains(&succ) {
                    s.succ.insert(succ.clone());
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
        self.top
            .iter()
            .filter(|&(_k, v)| v.num_prec == 0)
            .map(|(k, _v)| k.clone())
            .to_owned_vec()
    }

    fn delete_and_find(&mut self, prec: T) -> ~[T] {
        let mut result = ~[];
        do self.top.pop(&prec).map |p| {
            for s in p.succ.iter() {
                match self.top.find_mut(s) {
                    Some(y) => {
                        y.num_prec -= 1;
                        if y.num_prec == 0 {
                            result.push(s.clone());
                        }
                    }
                    None => {}
                }
            }
        };
        return result;
    }
}

fn tsort<T: Hash + IterBytes + Eq + Clone>(rels: &mut Relations<T>) -> ~[T] {
    let mut sorted = ~[];
    let mut queue = rels.find_all_not_preceded();
    while !queue.is_empty() {
        let prec = queue.shift();
        sorted.push(prec.clone());
        queue.push_all(rels.delete_and_find(prec));
    }
    return sorted;
}


pub fn solve() -> ~str {
    let result = io::file_reader(&Path::new("files/keylog.txt"))
        .map(|file| {
            let mut rels = Relations::new();
            do file.each_line |line| {
                let ds = line.iter().filter_map(|c| char::to_digit(c, 10)).to_owned_vec();
                for i in range(1, ds.len()) {
                    rels.set_dependant(ds[i - 1], ds[i]);
                }
                true
            };
            tsort(&mut rels)
        });

    match result {
        Err(msg) => fail!(msg),
        Ok(value) => return value.map(|d| d.to_str()).concat()
    }
}
