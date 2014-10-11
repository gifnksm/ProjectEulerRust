#![crate_name = "prob0079"]
#![crate_type = "rlib"]


use std::char;
use std::io::{BufferedReader, File};
use std::hash::Hash;
use std::collections::{HashMap, HashSet};

pub const EXPECTED_ANSWER: &'static str = "73162890";

struct Relation<T> {
    num_prec: uint,
    succ: HashSet<T>
}

impl<T: Hash + Eq> Relation<T> {
    fn new() -> Relation<T> { Relation { num_prec: 0, succ: HashSet::new() } }
}

struct Relations<T> {
    top: HashMap<T, Relation<T>>
}

impl<T: Hash + Eq + Clone> Relations<T> {
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

    fn find_all_not_preceded(&self) -> Vec<T> {
        self.top
            .iter()
            .filter(|&(_k, v)| v.num_prec == 0)
            .map(|(k, _v)| k.clone())
            .collect()
    }

    fn delete_and_find(&mut self, prec: T) -> Vec<T> {
        let mut result = Vec::new();
        self.top.pop(&prec).map(|p| {
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
        });
        result
    }
}

fn tsort<T: Hash + Eq + Clone>(rels: &mut Relations<T>) -> Vec<T> {
    let mut sorted = Vec::new();
    let mut queue = rels.find_all_not_preceded();
    while !queue.is_empty() {
        let prec = queue.remove(0).unwrap();
        sorted.push(prec.clone());
        queue.push_all(rels.delete_and_find(prec).as_slice());
    }
    sorted
}


pub fn solve() -> String {
    let mut br = BufferedReader::new(
        File::open(&Path::new("files/p079_keylog.txt")).ok().expect("file not found."));

    let mut rels = Relations::new();
    for line in br.lines().filter_map(|line| line.ok()) {
        let ds = line.as_slice().chars()
            .filter_map(|c| char::to_digit(c, 10)).collect::<Vec<uint>>();
        for i in range(1, ds.len()) {
            rels.set_dependant(ds[i - 1], ds[i]);
        }
    }
    tsort(&mut rels).iter().map(|d| d.to_string()).collect::<Vec<String>>().concat()
}
