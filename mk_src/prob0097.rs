#![crate_name = "prob0097"]
#![crate_type = "rlib"]

extern crate num;

use num::{One, Zero, BigUint};

pub const EXPECTED_ANSWER: &'static str = "8739992577";

#[inline(always)]
fn pow_unit(base: &BigUint, exp: &BigUint, unit: &BigUint) -> BigUint {
    let two: BigUint = FromPrimitive::from_uint(2).unwrap();
    let mut result = One::one();
    let mut itr = exp.clone();
    let mut pow = base.clone();
    while !itr.is_zero() {
        if itr % two == One::one() {
            result = mul_unit(&result, &pow, unit);
        }
        itr = itr >> One::one();
        pow = mul_unit(&pow, &pow, unit);
    }
    return result;
}

#[inline(always)]
fn mul_unit(a: &BigUint, b: &BigUint, unit: &BigUint) -> BigUint {
    (*a * *b) % *unit
}

#[inline(always)]
fn add_unit(a: &BigUint, b: &BigUint, unit: &BigUint) -> BigUint {
    (*a + *b) % *unit
}

pub fn solve() -> String {
    let unit: BigUint = FromPrimitive::from_uint(100_0000_0000).unwrap();
    return add_unit(
        &mul_unit(&FromPrimitive::from_uint(28433).unwrap(),
                  &pow_unit(&FromPrimitive::from_uint(2).unwrap(),
                            &FromPrimitive::from_uint(7830457).unwrap(),
                            &unit),
                  &unit),
        &One::one(),
        &unit
    ).to_string();
}
