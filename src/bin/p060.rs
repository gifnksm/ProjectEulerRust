//! [Problem 60](https://projecteuler.net/problem=60) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use prime::PrimeSet;
use std::collections::HashMap;

fn concat_num(n: u64, m: u64) -> u64 {
    let mut d = 1;
    while d <= m {
        d *= 10;
    }
    n * d + m
}

struct ConcatPrimeNums {
    ps: PrimeSet,
    iter: prime::Nums,
}

impl ConcatPrimeNums {
    fn new(ps: &PrimeSet) -> ConcatPrimeNums {
        ConcatPrimeNums {
            ps: ps.clone(),
            iter: ps.iter(),
        }
    }
}

impl Iterator for ConcatPrimeNums {
    type Item = (u64, Vec<u64>);

    fn next(&mut self) -> Option<(u64, Vec<u64>)> {
        let n = self.iter.next().unwrap();
        let pairs = self
            .ps
            .iter()
            .take_while(|&m| m <= n)
            .filter(|&m| {
                (n + m) % 3 != 0
                    && self.ps.contains(concat_num(n, m))
                    && self.ps.contains(concat_num(m, n))
            })
            .collect();
        Some((n, pairs))
    }
}

fn union_vec(v1: &[u64], v2: &[u64]) -> Vec<u64> {
    let mut result = Vec::new();
    let mut i1 = 0;
    let mut i2 = 0;
    let l1 = v1.len();
    let l2 = v2.len();
    while i1 < l1 && i2 < l2 {
        if v1[i1] < v2[i2] {
            i1 += 1;
            continue;
        }
        if v1[i1] > v2[i2] {
            i2 += 1;
            continue;
        }
        result.push(v1[i1]);
        i1 += 1;
        i2 += 1;
    }
    result
}

fn find_chain(pairs: &[u64], set: &[u64], map: &HashMap<u64, Vec<u64>>) -> Vec<Vec<u64>> {
    let mut result = Vec::new();

    for (i, &p) in pairs.iter().enumerate() {
        let union_pairs = union_vec(&pairs[..i], &map.get(&p).unwrap());
        let pset = {
            let mut v = vec![p];
            v.extend(set.iter().copied());
            v
        };
        if union_pairs.is_empty() {
            result.push(pset);
        } else {
            result.extend(find_chain(&union_pairs, &pset, map));
        }
    }

    result
}

fn compute(len: usize) -> Vec<u64> {
    let prime = PrimeSet::new();
    let mut map = HashMap::new();

    for (n, pairs) in ConcatPrimeNums::new(&prime) {
        if pairs.len() >= len {
            for set in find_chain(&pairs, &[n], &map) {
                if set.len() >= len {
                    return set;
                }
            }
        }
        let _ = map.insert(n, pairs);
    }

    unreachable!();
}

fn solve() -> String {
    compute(5).into_iter().sum::<u64>().to_string()
}

common::problem!("26033", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn concat_num() {
        assert_eq!(12345, super::concat_num(123, 45));
        assert_eq!(123, super::concat_num(123, 0));
        assert_eq!(123, super::concat_num(0, 123));
    }

    #[test]
    fn four() {
        assert_eq!(&[3, 7, 109, 673], &super::compute(4)[..]);
    }
}
