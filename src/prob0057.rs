#![crate_id = "prob0057"]
#![crate_id = "prob0057"]
#![crate_type = "rlib"]
#![crate_type = "rlib"]

extern crate num;

use std::mem;
use num::bigint::BigUint;

pub static EXPECTED_ANSWER: &'static str = "153";

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
struct Frac { nd: (BigUint, BigUint) }

impl Frac {
    fn new() -> Frac {
        Frac { nd: (FromPrimitive::from_uint(3).unwrap(), FromPrimitive::from_uint(2).unwrap()) }
    }
}

impl Iterator<(BigUint, BigUint)> for Frac {
    #[inline]
    fn next(&mut self) -> Option<(BigUint, BigUint)> {
        let next = {
            let (ref n, ref d) = self.nd;
            (((*d) << 1) + (*n), (*n) + (*d))
        };
        Some(mem::replace(&mut self.nd, next))
    }
}

pub fn solve() -> ~str {
    Frac::new()
        .take(1000)
        .count(|(n, d)| n.to_str().len() > d.to_str().len())
        .to_str()
}

#[cfg(test)]
mod tests {
    use num::bigint::BigUint;
    use super::Frac;

    #[test]
    fn frac() {
        fn gen(n: uint, d: uint) -> (BigUint, BigUint) {
            (FromPrimitive::from_uint(n).unwrap(),
             FromPrimitive::from_uint(d).unwrap())
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
