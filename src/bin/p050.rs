#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(phase, slicing_syntax)]

#[phase(plugin, link)] extern crate common;
extern crate prime;

use std::iter::AdditiveIterator;
use prime::PrimeSet;

fn get_longer(ps: &[u64], p: u64, min_len: uint) -> Option<uint> {
    let max_avg = if min_len == 0 { p } else { p / (min_len as u64) };

    let mut start = 0;
    let mut end   = min_len;
    let mut sum   = ps.iter().take(min_len).map(|&x| x).sum();

    loop {
        let len = end - start;
        if sum > max_avg * (len as u64) { return None; }

        match sum.cmp(&p) {
            Equal => {
                if len <= min_len {
                    return None
                }
                return Some(len)
            }
            Less => {
                sum += ps[end];
                end += 1;
            }
            Greater => {
                sum -= ps[start];
                start += 1;
            }
        }
    }
}

fn compute(limit: u64) -> (u64, uint) {
    let ps = PrimeSet::new()
        .iter()
        .take_while(|&p| p <= limit)
        .collect::<Vec<_>>();

    let mut len = 0;
    let mut num = 0;
    for &p in ps.iter() {
        if let Some(l) = get_longer(ps[], p, len) {
            len = l;
            num = p;
        }
    }
    (num, len)
}

fn solve() -> String {
    compute(1000000).0.to_string()
}

problem!("997651", solve);

#[cfg(test)]
mod tests {
    #[test] fn hundred() { assert_eq!((41, 6), super::compute(100)) }
    #[test] fn thounsand() { assert_eq!((953, 21), super::compute(1000)) }
}
