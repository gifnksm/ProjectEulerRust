//! [Problem 94](https://projecteuler.net/problem=94) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use cont_frac::PelRoots;
use num_bigint::BigUint;
use num_traits::ToPrimitive;

// S(a, b): triangle a-a-b's area
// S(a, b) = b * sqrt(4a^2-b^2) / 4
// S(a, a+1) = (a+1) * sqrt(4a^2 - (a+1)^2) / 4
//           = (a+1) * sqrt((a-1)(3a+1)) / 4
// S(a, a-1) = (a-1) * sqrt(4a^2 - (a-1)^2) / 4
//           = (a-1) * sqrt((a+1)(3a-1)) / 4
//
// if a is even, (a-1)(3a+1) and (a+1)(3a-1) is odd => S is not an integer.
// so, a is odd.
//
// a := 2k + 1
// S(a, a+1) = (a+1) * sqrt((a-1)(3a+1)) / 4
//           = (k+1) * sqrt(k(3k+2))
// S(a, a-1) = (a-1) * sqrt((a+1)(3a-1)) / 4
//           = k * sqrt((k+1)(3k+1))
//
// find k which k(3k + 1) is square of (k+1)(3k+1) is square
// k(3k+2) = n^2 => 3(k^2 + 2/3k) = n^2
//               => 3(k + 1/3)^2 - 1/3 = n^2
//               => (3k + 1)^2 - 3n^2 = 1
// (k + 1)(3k+1) = n^2 => 3k^2 + 4k + 1 = n^2
//                     => 3(k^2 + 2/3k) + 1 = n^2
//                     => 3(k + 2/3)^2 - 1/3 = n^2
//                     => (3k + 2)^2 - 3n^2 = 1
//
// solving diophantine x^2 - 3y^2 = 1
// side length L := a + a + b
//
// k = (x - 1) / 3
//   => L = 6k + 4 = 2x + 2
// k = (x - 2) / 3
//   => L = 6k + 2 = 2x - 2
//
// L <= 1000000000
// k <= (100000000 - 4) / 6, (100000000 - 2) / 6

fn solve() -> String {
    let limit = 1000000000;

    PelRoots::<BigUint>::new(3)
        .map(|(x, _y)| x.to_u32().unwrap())
        .map(|x| match x % 3 {
            1 => {
                let k = (x - 1) / 3;
                let a = 2 * k + 1;
                let b = a + 1;
                (a, b)
            }
            2 => {
                let k = (x - 2) / 3;
                let a = 2 * k + 1;
                let b = a - 1;
                (a, b)
            }
            _ => panic!(),
        })
        .filter(|&(_a, b)| b != 0)
        .map(|(a, b)| 2 * a + b)
        .take_while(|&side| side <= limit)
        .sum::<u32>()
        .to_string()
}

common::problem!("518408346", solve);
