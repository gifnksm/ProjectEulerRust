#[link(name = "prob0097", vers = "0.0")];
#[crate_type = "lib"];

extern mod extra;
extern mod common;

use std::num::{ One, Zero };
use extra::bigint::{ BigUint };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 97,
    answer: "8739992577",
    solver: solve
};

#[inline(always)]
fn pow_unit(base: &BigUint, exp: &BigUint, unit: &BigUint) -> BigUint {
    let two = BigUint::from_uint(2);
    let mut result = One::one();
    let mut itr = exp.clone();
    let mut pow = base.clone();
    while !itr.is_zero() {
        if itr % two == One::one() {
            result = mul_unit(&result, &pow, unit);
        }
        itr >>= One::one();
        pow = mul_unit(&pow, &pow, unit);
    }
    return result;
}

#[inline(always)]
fn mul_unit(a: &BigUint, b: &BigUint, unit: &BigUint) -> BigUint {
    (a * *b) % *unit
}

#[inline(always)]
fn add_unit(a: &BigUint, b: &BigUint, unit: &BigUint) -> BigUint {
    (a + *b) % *unit
}

pub fn solve() -> ~str {
    let unit = BigUint::from_uint(100_0000_0000);
    return add_unit(
        &mul_unit(&BigUint::from_uint(28433),
                  &pow_unit(&BigUint::from_uint(2), &BigUint::from_uint(7830457), &unit),
                  &unit),
        &One::one(),
        &unit
    ).to_str();
}
