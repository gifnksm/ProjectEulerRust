//! [Problem 35](https://projecteuler.net/problem=35) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results)]

#[macro_use(problem)] extern crate common;
extern crate integer;
extern crate prime;

use integer::Integer;
use prime::PrimeSet;

fn is_circular_prime(ps: &PrimeSet, n: u64) -> bool {
    let radix = 10;
    let ds = n.into_digits(radix).collect::<Vec<_>>();

    let mut buf = ds.clone();
    for i in (1 .. ds.len()) {
        for j in (0 .. buf.len()) {
            buf[j] = ds[(i + j) % ds.len()];
        }
        let circ = Integer::from_digits(buf.iter().map(|&x| x), radix);
        if !ps.contains(circ) {
            return false;
        }
    }

    true
}

fn compute(limit: u64) -> usize {
    let ps = PrimeSet::new();
    ps.iter()
        .take_while(|&p| p < limit)
        .filter(|&n| is_circular_prime(&ps, n))
        .count()
}

fn solve() -> String {
    compute(1000000).to_string()
}

problem!("55", solve);

#[cfg(test)]
mod tests {
    use prime::PrimeSet;

    #[test]
    fn is_circular_prime() {
        let ps = PrimeSet::new();
        assert_eq!(true, super::is_circular_prime(&ps, 197));
        assert_eq!(false, super::is_circular_prime(&ps, 21));
        assert_eq!(true, super::is_circular_prime(&ps, 2));
    }

    #[test]
    fn below100() {
        assert_eq!(13, super::compute(100));
    }
}
