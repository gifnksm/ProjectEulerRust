#![crate_name = "prob0109"]
#![crate_type = "rlib"]

extern crate math;

use math::poly::Poly;

pub const EXPECTED_ANSWER: &'static str = "38182";

pub fn solve() -> String {
    let mut single = Vec::from_elem(26, 0u);
    let mut double = Vec::from_elem(51, 0u);
    let mut triple = Vec::from_elem(61, 0u);
    let mut dup    = Vec::from_elem(121, 0u);
    for i in range(1u, 21) {
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

    let single = Poly::new(single);
    let double = Poly::new(double);
    let triple = Poly::new(triple);
    let dup    = Poly::new(dup);

    let p_all = single + double + triple;
    let p1    = double.clone();
    let p2    = double * p_all;
    let p3    = double * Poly::new((p_all * p_all + dup)
                                   .as_slice()
                                   .iter()
                                   .map(|n| n / 2)
                                   .collect());
    let total = p1 + p2 + p3;
    return total.as_slice().iter().take(100).fold(0, |i, &a| i + a).to_string();
}
