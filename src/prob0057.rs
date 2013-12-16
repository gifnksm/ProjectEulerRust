#[crate_type = "rlib"];

extern mod extra;

use std::util;
use extra::bigint::BigUint;

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
        Some(util::replace(&mut self.nd, next))
    }
}

pub fn solve() -> ~str {
    Frac::new().take(1000)
        .map(|(n, d)| (n.to_str().len(), d.to_str().len()))
        .count(|(n_len, d_len)| n_len > d_len)
        .to_str()
}
