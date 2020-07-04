//! [Problem 100](https://projecteuler.net/problem=100) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use cont_frac::PelNegRoots;
use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::One;

// b/s * (b-1)/(s-1) = 1/2
// 2b(b - 1) = s * (s-1)
// 2b^2 - 2b = s^2 - s
// 2(b - 1/2)^2 - 1/2 = (s - 1/2)^2 - 1/4
// 2(2b - 1)^2 - 2 = (2s - 1)^2 - 1
// (2s - 1)^2 - 2(2b - 1)^2 = -1
// x^2 - 2y = -1
// s = (x + 1) / 2
// b = (y + 1) / 2
fn compute(limit: BigUint) -> BigUint {
    let one = BigUint::one();
    PelNegRoots::<BigUint>::new(2)
        .filter(|&(ref x, ref y)| x.is_odd() && y.is_odd())
        .map(|(x, y)| ((x + &one) >> 1, (y + &one) >> 1))
        .find(|&(ref x, ref _y)| ((*x) >= limit))
        .map(|(_x, y)| y)
        .unwrap()
}

fn solve() -> String {
    let limit = "1000000000000".parse().unwrap();
    compute(limit).to_string()
}

common::problem!("756872327473", solve);

#[cfg(test)]
mod tests {
    use num_bigint::BigUint;
    use num_traits::FromPrimitive;

    #[test]
    fn twenty_one() {
        fn check(result: u32, total: u32) {
            let result: BigUint = FromPrimitive::from_u32(result).unwrap();
            let total: BigUint = FromPrimitive::from_u32(total).unwrap();
            assert_eq!(result, super::compute(total));
        }
        check(15, 21);
        check(85, 22);
    }
}
