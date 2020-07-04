//! [Problem 57](https://projecteuler.net/problem=57) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use num_bigint::BigUint;
use num_traits::FromPrimitive;
use std::mem;

// FIXME: Use cont_frac?

// a[0] = 1 + 1/2
// a[1] = 1 + 1/(2 + 1/2)
//      = 1 + 1/(1 + a[0]) =
// a[2] = 1 + 1/(2 + 1/(2 + 1/2))
//      = 1 + 1/(1 + a[1])
// a[i+1] = n[i+1] / d[i+1]
//        = 1 + 1 / (1 + n[i]/d[i])
//        = 1 + d[i] / (d[i] + n[i])
//        = (2d[i] + n[i]) / (d[i] + n[i])
//  n[0] = 3, d[0] = 2
//  n[i+1] = 2d[i] + n[i]
//  d[i+1] = d[i] + n[i]
struct Frac {
    nd: (BigUint, BigUint),
}

impl Frac {
    fn new() -> Frac {
        Frac {
            nd: (
                FromPrimitive::from_usize(3).unwrap(),
                FromPrimitive::from_usize(2).unwrap(),
            ),
        }
    }
}

impl Iterator for Frac {
    type Item = (BigUint, BigUint);

    #[inline]
    fn next(&mut self) -> Option<(BigUint, BigUint)> {
        let next = {
            let (ref n, ref d) = self.nd;
            ((d << 1) + n, n + d)
        };
        Some(mem::replace(&mut self.nd, next))
    }
}

fn solve() -> String {
    Frac::new()
        .take(1000)
        .filter(|&(ref n, ref d)| n.to_string().len() > d.to_string().len())
        .count()
        .to_string()
}

common::problem!("153", solve);

#[cfg(test)]
mod tests {
    use super::Frac;
    use num_bigint::BigUint;
    use num_traits::FromPrimitive;

    #[test]
    fn frac() {
        fn gen(n: u32, d: u32) -> (BigUint, BigUint) {
            (
                FromPrimitive::from_u32(n).unwrap(),
                FromPrimitive::from_u32(d).unwrap(),
            )
        }
        let mut it = Frac::new();
        assert_eq!(Some(gen(3, 2)), it.next());
        assert_eq!(Some(gen(7, 5)), it.next());
        assert_eq!(Some(gen(17, 12)), it.next());
        assert_eq!(Some(gen(41, 29)), it.next());
        assert_eq!(Some(gen(99, 70)), it.next());
        assert_eq!(Some(gen(239, 169)), it.next());
        assert_eq!(Some(gen(577, 408)), it.next());
        assert_eq!(Some(gen(1393, 985)), it.next());
    }
}
