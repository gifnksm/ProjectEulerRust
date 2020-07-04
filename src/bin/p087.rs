//! [Problem 87](https://projecteuler.net/problem=87) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use prime::PrimeSet;

fn compute(limit: u64) -> u32 {
    let prime = PrimeSet::new();
    let mut cnt = 0;
    let mut set = vec![false; limit as usize];

    for p in &prime {
        let p4 = p * p * p * p;
        if p4 >= limit {
            break;
        }

        for q in &prime {
            let q3 = q * q * q;
            if p4 + q3 >= limit {
                break;
            }

            for r in &prime {
                let r2 = r * r;
                let s = p4 + q3 + r2;
                if s >= limit {
                    break;
                }

                if set[s as usize] {
                    continue;
                }
                set[s as usize] = true;
                cnt += 1;
            }
        }
    }

    cnt
}

fn solve() -> String {
    compute(50000000).to_string()
}

common::problem!("1097343", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn fifty() {
        assert_eq!(4, super::compute(50));
    }
}
