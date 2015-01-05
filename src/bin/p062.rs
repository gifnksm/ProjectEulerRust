//! [Problem 62](https://projecteuler.net/problem=62) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(default_type_params, phase)]

#[phase(plugin, link)] extern crate common;
extern crate integer;

use std::collections::{HashSet, HashMap};
use std::collections::hash_map::Entry;
use std::iter;
use integer::Integer;

fn perm_exact(cnt: uint) -> HashSet<uint> {
    let mut map = HashMap::<_, (uint, uint)>::new();
    let mut set = HashSet::new();

    let mut limit = 10;
    for n in iter::count(1, 1) {
        let cube = n * n * n;
        if cube >= limit {
            if !set.is_empty() {
                return set
            }

            map.clear();
            limit *= 10
        }

        let ds = cube.into_digit_histogram().to_vec();

        let (n, c) = match map.entry(&ds) {
            Entry::Occupied(e) => {
                let &(n, ref mut c) = e.into_mut();
                *c += 1;
                (n, *c)
            }
            Entry::Vacant(e) => {
                let _ = e.insert((cube, 1));
                (cube, 1)
            }
        };

        if c == cnt {
            set.insert(n);
        } else if c == cnt + 1 {
            set.remove(&n);
        }
    }

    unreachable!()
}

fn solve() -> String {
    perm_exact(5).iter().min().unwrap().to_string()
}

problem!("127035954683", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn three_perm() {
        assert_eq!(41063625, *super::perm_exact(3).iter().min().unwrap());
    }
}
