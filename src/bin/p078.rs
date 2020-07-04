//! [Problem 78](https://projecteuler.net/problem=78) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

const MILLION: i32 = 1000000;

fn penta(n: i32) -> i32 {
    n * (3 * n - 1) / 2
}

fn solve() -> String {
    let mut v = [0; 65536];
    v[0] = 1;

    for n in 1.. {
        let mut way = 0;

        for i in 0.. {
            let k = i % 4;
            let p = if k == 0 || k == 2 {
                penta(i / 2 + 1)
            } else {
                penta(-i / 2 - 1)
            };
            if p > n {
                break;
            }

            let idx = (n - p) as usize;

            way = match k {
                0 => way + v[idx],
                1 => way + v[idx],
                2 => way - v[idx],
                _ => way - v[idx],
            } % MILLION
        }
        v[n as usize] = way;

        if way == 0 {
            return n.to_string();
        }
    }

    unreachable!()
}

common::problem!("55374", solve);
