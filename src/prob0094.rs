#[link(name = "prob0094", vers = "0.0")];
#[crate_type = "lib"];

extern mod extra;
extern mod math;

use extra::bigint::BigUint;
use math::cont_frac;

pub static EXPECTED_ANSWER: &'static str = "518408346";

// triangle a-a-b's area
// S(a, b) := b * sqrt(4a^2-b^2) / 4
// S(a, a+1) = (a+1) * sqrt(4a^2 - (a+1)^2) / 4
// S(a, a-1) = (a-1) * sqrt(4a^2 - (a-1)^2) / 4
//
// if a is even, (a-1)(3a+1) and (a+1)(3a-1) is odd => S is not an integer.
// so, a is odd.
// a := 2k + 1, b = 2k+2, 2k
// S+ = (a+1) * sqrt((a-1)(3a+1)) / 4
//    = (k+1) * sqrt(k(3k+2))
// S- = (a-1) * sqrt((a+1)(3a-1)) / 4
//    = k * sqrt((k+1)(3k+1))
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
//
// side length L := a + a + b = 6k+4, 6k+2
// L <= 1000000000
// k <= (100000000 - 4) / 6, (100000000 - 2) / 6
fn each_ab(f: &fn(uint, uint) -> bool) -> bool {
    do cont_frac::each_pel::<BigUint>(3) |x, _y| {
        match x.to_uint().unwrap() % 3 {
            1 => {
                let k = (x.to_uint().unwrap() - 1) / 3;
                let a = 2 * k + 1;
                let b = a + 1;
                f(a, b)
            }
            2 => {
                let k = (x.to_uint().unwrap() - 2) / 3;
                let a = 2 * k + 1;
                let b = a - 1;
                f(a, b)
            }
            _ => fail!()
        }
    }
}

pub fn solve() -> ~str {
    let limit = 1000000000;
    let mut total = 0;

    do each_ab |a, b| {
        if b == 0 {
            true
        } else {
            let side = a + a + b;
            if side > limit {
                false
            } else {
                total += side;
                true
            }
        }
    };

    return total.to_str();
}
