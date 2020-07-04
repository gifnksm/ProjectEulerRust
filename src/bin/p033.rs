//! [Problem 33](https://projecteuler.net/problem=33) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use num_integer::Integer;

// AB / AC => NG (10A+B : 10A+C = B : C => 10AC+BC = 10AB+BC => 10A(C-B) = 0 -> trivial)
// BA / CA => NG
// AB / CA => (10A + B : 10C + A = B : C => 10AC + BC = 10BC + AB => A(10C-B) = 9BC)
// BA / AC => (10B + A : 10A + C = B : C => 10BC + AC = 10AB + BC => A(10B-C) = 9BC)
//
// * Pattern 1: AB / CA = B / C
// A = 9BC / (10C - B)
// C > B
//
// * Pattern 2: BA / AC = B / C
// A = 9BC / (10B - C)
// C > B

fn compute() -> u32 {
    let mut prod_numer = 1;
    let mut prod_denom = 1;

    for b in 1u32..10 {
        for c in (b + 1)..10 {
            // Pattern 1
            let a_numer = 9 * b * c;
            let a_denom = 10 * c - b;
            if a_numer % a_denom == 0 && a_numer < 10 * a_denom {
                prod_numer *= b;
                prod_denom *= c;
            }
            // Pattern 2
            let a_numer = 9 * b * c;
            let a_denom = 10 * b - c;
            if a_numer % a_denom == 0 && a_numer < 10 * a_denom {
                prod_numer *= b;
                prod_denom *= c;
            }
        }
    }

    let gcd = prod_numer.gcd(&prod_denom);
    prod_denom / gcd
}

fn solve() -> String {
    compute().to_string()
}

common::problem!("100", solve);
