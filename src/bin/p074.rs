//! [Problem 74](https://projecteuler.net/problem=74) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(phase)]

#[phase(plugin, link)] extern crate common;
use std::collections::HashMap;

#[deriving(Clone)]
enum Length { Loop(uint), Chain(uint), Unknown }

fn fact_sum(mut n: uint, fs: &[uint, ..10]) -> uint {
    if n == 0 { return 1; }

    let mut sum = 0;
    while n > 0 {
        sum += fs[n % 10];
        n /= 10;
    }
    sum
}

fn get_chain_len(n: uint, map: &mut[Length], fs: &[uint, ..10]) -> uint {
    let mut chain_map = HashMap::new();
    let mut idx = n;
    let mut chain_len = 0;
    let mut loop_len  = 0;

    loop {
        match map[idx] {
            Length::Loop(c)  => { loop_len += c;  break; }
            Length::Chain(c) => { chain_len += c; break; }
            Length::Unknown  => {
                match chain_map.get(&idx) {
                    Some(&chain_idx) => {
                        loop_len  = chain_len - chain_idx;
                        chain_len = chain_idx;
                        break;
                    }
                    None => {
                        let _ = chain_map.insert(idx, chain_len);
                        idx = fact_sum(idx, fs);
                        chain_len += 1;
                    }
                }
            }
        }
    }

    for (&key, &idx) in chain_map.iter() {
        if idx >= chain_len {
            map[key] = Length::Loop(loop_len);
        } else {
            map[key] = Length::Chain(loop_len + chain_len - idx);
        }
    }

    chain_len + loop_len
}

fn solve() -> String {
    let limit = 1000000;
    let factorial = {
        let mut val = [1, ..10];
        for i in range(1u, 10) {
            val[i] = val[i - 1] * i;
        }
        val
    };

    let mut map = Vec::from_elem(factorial[9] * 6 + 1, Length::Unknown);
    let mut cnt = 0u;
    for n in range(1u, limit + 1) {
        let len = get_chain_len(n, map.as_mut_slice(), &factorial);
        if len == 60 { cnt += 1; }
    }

    cnt.to_string()
}

problem!("402", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn len() {
        let factorial = {
            let mut val = [1, ..10];
            for i in range(1u, 10) {
                val[i] = val[i - 1] * i;
            }
            val
        };
        let mut map = Vec::from_elem(factorial[9] * 6 + 1, super::Length::Unknown);

        assert_eq!(3, super::get_chain_len(169, map.as_mut_slice(), &factorial));
        assert_eq!(2, super::get_chain_len(871, map.as_mut_slice(), &factorial));
        assert_eq!(2, super::get_chain_len(872, map.as_mut_slice(), &factorial));
        assert_eq!(5, super::get_chain_len(69, map.as_mut_slice(), &factorial));
        assert_eq!(4, super::get_chain_len(78, map.as_mut_slice(), &factorial));
        assert_eq!(2, super::get_chain_len(540, map.as_mut_slice(), &factorial));
    }

}
