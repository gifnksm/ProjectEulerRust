#![crate_id = "prob0060"]
#![crate_type = "rlib"]

extern crate collections;
extern crate math;

use std::iter::AdditiveIterator;
use collections::HashMap;
use math::prime::{Prime, PrimeIterator};

pub static EXPECTED_ANSWER: &'static str = "26033";

fn concat_num(n: uint, m: uint) -> uint {
    let mut d = 1;
    while d <= m { d *= 10; }
    n * d + m
}

struct ConcatPrimeIterator {
    prime: Prime,
    iter: PrimeIterator
}

impl ConcatPrimeIterator {
    #[inline]
    fn new(prime: &Prime) -> ConcatPrimeIterator {
        ConcatPrimeIterator {
            prime: prime.clone(),
            iter: prime.iter()
        }
    }
}

impl Iterator<(uint, Vec<uint>)> for ConcatPrimeIterator {
    #[inline]
    fn next(&mut self) -> Option<(uint, Vec<uint>)> {
        let n = self.iter.next().unwrap();
        let pairs = self.prime.iter()
            .take_while(|&m| m <= n)
            .filter(|&m| (n + m) % 3 != 0 && {
                self.prime.contains(concat_num(n, m)) &&
                    self.prime.contains(concat_num(m, n))
            }).collect();
        Some((n, pairs))
    }
}

fn union_vec(v1: &[uint], v2: &[uint]) -> Vec<uint> {
    let mut result = Vec::new();
    let mut i1 = 0;
    let mut i2 = 0;
    let l1 = v1.len();
    let l2 = v2.len();
    while i1 < l1 && i2 < l2 {
        if v1[i1] < v2[i2] { i1 += 1; continue }
        if v1[i1] > v2[i2] { i2 += 1; continue }
        result.push(v1[i1]);
        i1 += 1;
        i2 += 1;
    }
    result
}

fn find_chain(pairs: &[uint], set: Vec<uint>, map: &HashMap<uint, Vec<uint>>) -> Vec<Vec<uint>> {
    let mut result = Vec::new();

    for (i, &p) in pairs.iter().enumerate() {
        let union_pairs = union_vec(pairs.slice(0, i), map.find(&p).unwrap().as_slice());
        let pset = vec!(p).append(set.as_slice());
        if union_pairs.is_empty() {
            result.push(pset);
        } else {
            result.push_all(find_chain(union_pairs.as_slice(), pset, map).as_slice());
        }
    }

    result
}

pub fn solve() -> StrBuf {
    let prime = Prime::new();
    let len = 5;
    let mut map = HashMap::new();

    for (n, pairs) in ConcatPrimeIterator::new(&prime) {
        if pairs.len() >= len {
            let sets = find_chain(pairs.as_slice(), vec![n], &map);
            for set in sets.iter() {
                if set.len() >= len {
                    return set.iter().map(|&x| x).sum().to_str();
                }
            }
        }
        map.insert(n, pairs);
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::concat_num;
    #[test]
    fn test_concat_num() {
        assert_eq!(12345, concat_num(123, 45));
        assert_eq!(123, concat_num(123, 0));
        assert_eq!(123, concat_num(0, 123));
    }
}
