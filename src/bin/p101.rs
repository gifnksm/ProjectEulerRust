//! [Problem 101](https://projecteuler.net/problem=101) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::{FromPrimitive, One, Zero};
use polynomial::Polynomial;

fn u(n: BigInt) -> BigInt {
    let mut sum = BigInt::zero();
    let mut prod = BigInt::one();
    for _ in 0..11 {
        sum += &prod;
        prod = &prod * (-&n);
    }
    sum
}

// Lagrange Interpolating with Naville's algorithm
fn op(ns: &[(BigInt, BigInt)]) -> Polynomial<BigInt> {
    let mut poly = Polynomial::new(vec![]);

    for (i, &(ref xi, ref yi)) in ns.iter().enumerate() {
        let mut term = Polynomial::new(vec![Ratio::from_integer(yi.clone())]);

        for (j, &(ref xj, ref _yj)) in ns.iter().enumerate() {
            if i == j {
                continue;
            }

            term = term
                * Polynomial::new(vec![
                    Ratio::new(-xj, xi - xj),
                    Ratio::new(One::one(), xi - xj),
                ]);
        }
        poly = poly + term;
    }

    let data = poly.data().iter().map(Ratio::to_integer).collect();
    Polynomial::new(data)
}

fn bop(ns: &[(BigInt, BigInt)]) -> BigInt {
    op(ns).eval(FromPrimitive::from_usize(ns.len() + 1).unwrap())
}

fn u_to_vec(dim: u32, f: fn(BigInt) -> BigInt) -> Vec<(BigInt, BigInt)> {
    (0..(dim + 1))
        .map(|i| {
            let n: BigInt = FromPrimitive::from_u32(i + 1).unwrap();
            (n.clone(), f(n))
        })
        .collect()
}

fn solve() -> String {
    let un = u_to_vec(10, u);
    (0..10)
        .map(|i| bop(&un[..i + 1]))
        .fold(num_traits::zero::<BigInt>(), |acc, elt| acc + elt)
        .to_string()
}

common::problem!("37076114526", solve);

#[cfg(test)]
mod tests {
    use num_bigint::BigInt;
    use num_traits::ToPrimitive;

    #[test]
    fn op() {
        fn u(n: BigInt) -> BigInt {
            &n * &n * &n
        }
        let un = super::u_to_vec(3, u);
        assert_eq!("1", super::op(&un[..1]).pretty("n"));
        assert_eq!("-6+7*n", super::op(&un[..2]).pretty("n"));
        assert_eq!("6-11*n+6*n^2", super::op(&un[..3]).pretty("n"));
        assert_eq!("n^3", super::op(&un).pretty("n"));
    }

    #[test]
    fn bop() {
        fn u(n: BigInt) -> BigInt {
            &n * &n * &n
        }
        let un = super::u_to_vec(3, u);
        assert_eq!(1, super::bop(&un[..1]).to_i32().unwrap());
        assert_eq!(15, super::bop(&un[..2]).to_i32().unwrap());
        assert_eq!(58, super::bop(&un[..3]).to_i32().unwrap());
    }
}
