//! [Problem 53](https://projecteuler.net/problem=53) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

// nCr-1 = r/(n-r+1) nCr
// nCr = n/(n-r) n-1Cr
// nC(r+1) = (n-r)/(r+1) nCr
fn compute() -> u32 {
    let limit = 1000000;

    let mut r = 0;
    let mut c = 1;
    let mut cnt = 0;
    for n in 1u32..101 {
        c = c * n / (n - r); // nCr

        if c < limit {
            while c < limit {
                if r == (n + 1) / 2 {
                    break;
                }
                c = c * (n - r) / (r + 1);
                r += 1;
            }
            if c < limit {
                continue;
            }
        } else {
            while c * r / (n - r + 1) >= limit {
                c = c * r / (n - r + 1);
                r -= 1;
            }
        }

        cnt += ((n - r) - r) + 1;
    }

    cnt
}

fn solve() -> String {
    compute().to_string()
}

common::problem!("4075", solve);
