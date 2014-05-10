#![crate_id = "prob0109"]
#![crate_type = "rlib"]

extern crate math;
use std::num::Zero;
use math::poly;

pub static EXPECTED_ANSWER: &'static str = "38182";

fn square<T: Zero + Add<T, T> + Mul<T, T>>(n: &[T]) -> Vec<T> { poly::mul(n, n) }

pub fn solve() -> ~str {
    let mut single = Vec::from_elem(26, 0);
    let mut double = Vec::from_elem(51, 0);
    let mut triple = Vec::from_elem(61, 0);
    let mut dup    = Vec::from_elem(121, 0);
    for i in range(1u, 21) {
        *single.get_mut(1 * i) = 1;
        *double.get_mut(2 * i) = 1;
        *triple.get_mut(3 * i) = 1;
        *dup.get_mut(2 * i) += 1;
        *dup.get_mut(4 * i) += 1;
        *dup.get_mut(6 * i) += 1;
    }
    *single.get_mut(25) = 1;
    *double.get_mut(50) = 1;
    *dup.get_mut(50)    += 1;
    *dup.get_mut(100)   += 1;

    let p_all = poly::add(poly::add(single.as_slice(), double.as_slice()).as_slice(),
                          triple.as_slice());
    let p1    = double.clone();
    let p2    = poly::mul(p_all.as_slice(), double.as_slice());
    let p3    = poly::mul(poly::add(square(p_all.as_slice()).as_slice(), dup.as_slice())
                          .iter().map(|n| n / 2).collect::<Vec<int>>().as_slice(),
                          double.as_slice());
    let total = poly::add(poly::add(p1.as_slice(), p2.as_slice()).as_slice(), p3.as_slice());
    return total.iter().take(100).fold(0, |i, &a| i + a).to_str();
}
