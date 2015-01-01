//! [Problem 95](https://projecteuler.net/problem=95) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(phase, slicing_syntax)]

#[phase(plugin, link)] extern crate common;

use std::iter;

fn get_chain_len(mut n: uint, len_map: &mut [Option<uint>], div_map: &[uint]) -> uint {
    if let Some(x) = len_map[n] {
        return x
    }

    let mut itr_map = vec![n];
    loop {
        n = div_map[n];

        if n >= len_map.len() {
            for &n in itr_map.iter() { len_map[n] = Some(0); }
            return 0;
        }

        match itr_map.position_elem(&n) {
            Some(idx) => {
                let len = itr_map.len() - idx;
                for &m in itr_map[.. idx].iter() { len_map[m] = Some(0); }
                for &m in itr_map[idx ..].iter() { len_map[m] = Some(len); }
                return len_map[itr_map[0]].unwrap()
            }
            None => { itr_map.push(n); }
        }
    }
}

fn create_proper_divisor_map(limit: uint) -> Vec<uint> {
    let mut map = Vec::from_elem(limit + 1, 1);
    map[0] = 0;
    map[1] = 1;
    for f in range(2, limit / 2) {
        for n in iter::range_step(2 * f, limit, f) {
            map[n] += f;
        }
    }
    map
}

fn compute(limit: uint) -> uint {
    let mut len_map = Vec::from_elem(limit + 1, None);
    let div_map = create_proper_divisor_map(limit);

    let (n, _) = range(1, len_map.len())
        .map(|n| (n, get_chain_len(n, len_map.as_mut_slice(), div_map[])))
        .max_by(|&(_, div)| div)
        .unwrap();

    n
}

fn solve() -> String {
    compute(1000000).to_string()
}

problem!("14316", solve);
