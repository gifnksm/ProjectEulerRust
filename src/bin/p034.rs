//! [Problem 34](https://projecteuler.net/problem=34) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(phase)]

#[phase(plugin, link)] extern crate common;

fn compute() -> uint {
    let mut facts: [uint, ..10] = [ 0, ..10 ];
    facts[0] = 1;
    for i in range(1, facts.len()) {
        facts[i] = facts[i - 1] * i;
    }

    let mut answer = 0;
    for n in range(0, facts[9].to_string().len() * facts[9]) {
        let mut itr = n;
        let mut sum = 0;
        while itr > 0 {
            sum += facts[itr % 10];
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

problem!("40730", solve);
