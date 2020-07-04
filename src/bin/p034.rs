//! [Problem 34](https://projecteuler.net/problem=34) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

fn compute() -> u32 {
    let mut facts: [u32; 10] = [0; 10];
    facts[0] = 1;
    for i in 1..facts.len() {
        facts[i] = facts[i - 1] * (i as u32);
    }

    let mut answer = 0;
    for n in 0..((facts[9].to_string().len() as u32) * facts[9]) {
        let mut itr = n;
        let mut sum = 0;
        while itr > 0 {
            sum += facts[(itr % 10) as usize];
            itr /= 10;
        }
        if sum == n {
            answer += sum;
        }
    }
    answer - 1 - 2
}

fn solve() -> String {
    compute().to_string()
}

common::problem!("40730", solve);
