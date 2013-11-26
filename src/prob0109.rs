#[link(name = "prob0109", vers = "0.0", package_id = "prob0109")];
#[crate_type = "lib"];

extern mod math;
use std::vec;
use std::num::Zero;
use math::poly;

pub static EXPECTED_ANSWER: &'static str = "38182";

fn square<T: Zero + Add<T, T> + Mul<T, T>>(n: &[T]) -> ~[T] { poly::mul(n, n) }

pub fn solve() -> ~str {
    let mut single = vec::from_elem(26, 0);
    let mut double = vec::from_elem(51, 0);
    let mut triple = vec::from_elem(61, 0);
    let mut dup    = vec::from_elem(121, 0);
    for i in range(1, 21) {
        single[1 * i] = 1;
        double[2 * i] = 1;
        triple[3 * i] = 1;
        dup[2 * i] += 1;
        dup[4 * i] += 1;
        dup[6 * i] += 1;
    }
    single[25] = 1;
    double[50] = 1;
    dup[50]    += 1;
    dup[100]   += 1;

    let p_all = poly::add(poly::add(single, double), triple);
    let p1    = double.clone();
    let p2    = poly::mul(p_all, double);
    let p3    = poly::mul(poly::add(square(p_all), dup).map(|n| n / 2), double);
    let total = poly::add(poly::add(p1, p2), p3);
    return total.iter().take(100).fold(0, |i, &a| i + a).to_str();
}
