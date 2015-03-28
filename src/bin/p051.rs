//! [Problem 51](https://projecteuler.net/problem=51) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results)]

#[macro_use(problem)] extern crate common;
extern crate integer;
extern crate prime;

use integer::Integer;
use prime::PrimeSet;

fn compute(num_value: usize) -> u64 {
    let radix = 10;
    let ps = PrimeSet::new();

    for p in &ps {
        let ds = p.into_digits(radix as u64);
        let hs = p.into_digit_histogram();

        for (d_src, &cnt) in hs.iter().enumerate() {
            // Skip digits that are appeared less than twice.
            if cnt <= 1 { continue }

            let mut num_prime = 1;
            for d_dst in (d_src + 1 .. radix) {
                if radix - d_dst < num_value - num_prime { break }

                let it = ds.clone().map(|d| if d == (d_src as u64) { d_dst as u64 } else { d });
                if ps.contains(Integer::from_digits(it, radix as u64)) {
                    num_prime += 1;
                }
            }

            if num_prime >= num_value {
                return p
            }
        }
    }
    unreachable!()
}

fn solve() -> String {
    compute(8).to_string()
}

problem!("121313", solve);

#[cfg(test)]
mod tests {
    #[test] fn seven() { assert_eq!(56003, super::compute(7)) }
}
