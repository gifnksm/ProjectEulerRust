//! [Problem 106](https://projecteuler.net/problem=106) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use prime::PrimeSet;
use std::collections::HashMap;

// n = 4
// (i, j): i: size of B, j: size of C
// (1,1) => 4C2 * (2C1 / 2) =  6
// (1,2) => 4C3 * 3C1       = 12
// (1,3) => 4C4 * 4C1       =  4
// (2,2) => 4C4 * (4C2 / 2) =  3
//                        ==> 25

// n = 7
// (1,1) => 7C2 * (2C1 / 2) = 21 * (2/2)  =  21
// (1,2) => 7C3 * 3C1       = 35 * 3      = 105
// (1,3) => 7C4 * 4C1       = 35 * 4      = 140
// (1,4) => 7C5 * 5C1       = 21 * 5      = 105
// (1,5) => 7C6 * 6C1       =  7 * 6      =  42
// (1,6) => 7C7 * 7C1       =  1 * 7      =   7
// (2,2) => 7C4 * (4C2/ 2)  = 35 * (6/2)  = 105
// (2,3) => 7C5 * 5C2       = 21 * 10     = 210
// (2,4) => 7C6 * 6C2       =  7 * 15     = 105
// (2,5) => 7C7 * 7C2       =  1 * 21     =  21
// (3,3) => 7C6 * (6C3 / 2) =  7 * (20/2) =  70
// (3,4) => 7C7 * 7C3       =  1 * 35     =  35
//                                      ==> 966

// 7 => (2,2)
// B B C C => S(B) < S(C)
// B C B C => S(B) < S(C)
// B C C B => ??
// 1 * 7C4 = 35

// 7 => (3,3)
// B B B C C C => S(B) < S(C)
// B B C B C C => S(B) < S(C)
// B B C C B C => S(B) < S(C)
// B B C C C B => ??
// B C B B C C => S(B) < S(C)
// B C B C B C => S(B) < S(C)
// B C B C C B => ??
// B C C B B C => ??
// B C C B C B => ??
// B C C C B B => ??
// 5 * 7C6 = 35

// f(i,j) := number of the pairs which satisfy S(B) < S(C)
// f(i,i) = f(i-1,i)
// f(i,j) if i < j = f(i,j-1) + f(i-1,j)
// f(i,j) if i > j = 0
// f(0,0) = 0
// f(i,0) = 0 if i > 0
// f(0,j) = 1 if j > 0

// f(1,1) = f(0,1)
//        = 1
// f(2,2) = f(1,2)
//        = f(0,2) + f(1,1)
//        = 1      + 1
//        = 2
// f(3,3) = f(2,3)
//        = f(2,2) + f(1,3)
//        = 2      + f(1,2) + f(0,3)
//        = 2      + 2      + 1
//        = 5

// N(A) = n
// N(B) = N(C) = k
// => nC2k * (2kCk/2 - f(k,k)) pairs
// answer: \sum_{k=1}^{n/2} (nC2k * (2kCk/2 - f(k,k)))

fn f(i: u64, j: u64, map: &mut HashMap<(u64, u64), u64>) -> u64 {
    match (i, j) {
        (0, 0) => return 0,
        (_, 0) => return 0,
        (0, _) => return 1,
        _ if i == j => return f(i - 1, j, map),
        _ if i > j => return 0,
        _ => {}
    }

    if let Some(n) = map.get(&(i, j)) {
        return *n;
    }

    let val = f(i, j - 1, map) + f(i - 1, j, map);
    let _ = map.insert((i, j), val);
    val
}

fn get_num_pairs(ps: &PrimeSet, n: u64) -> u64 {
    let mut map = HashMap::new();
    (1..n / 2 + 1)
        .map(|k| ps.combination(n, 2 * k) * (ps.combination(2 * k, k) / 2 - f(k, k, &mut map)))
        .sum()
}

fn solve() -> String {
    get_num_pairs(&PrimeSet::new(), 12).to_string()
}

common::problem!("21384", solve);

#[cfg(test)]
mod tests {
    use prime::PrimeSet;
    use std::collections::HashMap;

    #[test]
    fn test_f() {
        let mut map = HashMap::new();
        assert_eq!(1, super::f(1, 1, &mut map));
        assert_eq!(2, super::f(2, 2, &mut map));
        assert_eq!(5, super::f(3, 3, &mut map));
        assert_eq!(14, super::f(4, 4, &mut map));
        assert_eq!(42, super::f(5, 5, &mut map));
    }
    // f(4,4) = f(3,4)
    //        = f(3,3) + f(2,4)
    //        = 5      + f(2,3) + f(1,4)
    //        = 5      + 5      + f(1,3) + f(0,4)
    //        = 5      + 5      + 3      + 1
    //        = 14
    // f(5,5) = f(4,5)
    //        = f(4,4) + f(3,5)
    //        = 14     + f(3,4) + f(2,5)
    //        = 14     + 14     + f(2,4) + f(1,5)
    //        = 14     + 14     + 9      + f(1,4) + f(0,5)
    //        = 14     + 14     + 9      + 4      + 1
    //        = 42

    #[test]
    fn test_get_num_pairs() {
        let prime = PrimeSet::new();
        assert_eq!(1, super::get_num_pairs(&prime, 4));
        assert_eq!(70, super::get_num_pairs(&prime, 7));
    }
}
