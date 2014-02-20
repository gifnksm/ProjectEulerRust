#[crate_id = "prob0060"];
#[crate_type = "rlib"];

extern crate math;

use std::hashmap::HashMap;
use std::iter::AdditiveIterator;
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

impl Iterator<(uint, ~[uint])> for ConcatPrimeIterator {
    #[inline]
    fn next(&mut self) -> Option<(uint, ~[uint])> {
        let n = self.iter.next().unwrap();
        let pairs = self.prime.iter()
            .take_while(|&m| m <= n)
            .filter(|&m| (n + m) % 3 != 0 && {
                self.prime.contains(concat_num(n, m)) &&
                    self.prime.contains(concat_num(m, n))
            }).to_owned_vec();
        Some((n, pairs))
    }
}

fn union_vec(v1: &[uint], v2: &[uint]) -> ~[uint] {
    let mut result = ~[];
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

fn find_chain(pairs: &[uint], set: ~[uint], map: &HashMap<uint, ~[uint]>) -> ~[~[uint]] {
    let mut result = ~[];

    for (i, &p) in pairs.iter().enumerate() {
        let union_pairs = union_vec(pairs.slice(0, i), *map.find(&p).unwrap());
        if union_pairs.is_empty() {
            result.push(~[p] + set);
        } else {
            result.push_all(find_chain(union_pairs, ~[p] + set, map));
        }
    }

    result
}

pub fn solve() -> ~str {
    let prime = Prime::new();
    let len = 5;
    let mut map = HashMap::new();

    for (n, pairs) in ConcatPrimeIterator::new(&prime) {
        if pairs.len() >= len {
            let sets = find_chain(pairs, ~[n], &map);
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
