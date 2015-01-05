//! [Problem 97](https://projecteuler.net/problem=97) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(phase)]

#[phase(plugin, link)] extern crate common;
extern crate num;

use std::num::FromPrimitive;
use num::{One, Zero, BigUint};

fn pow_unit(base: &BigUint, exp: &BigUint, unit: &BigUint) -> BigUint {
    let two: BigUint = FromPrimitive::from_uint(2).unwrap();
    let mut result = One::one();
    let mut itr = exp.clone();
    let mut pow = base.clone();
    while !itr.is_zero() {
        if &itr % &two == One::one() {
            result = mul_unit(&result, &pow, unit);
        }
        itr = itr >> One::one();
        pow = mul_unit(&pow, &pow, unit);
    }
    return result;
}

fn mul_unit(a: &BigUint, b: &BigUint, unit: &BigUint) -> BigUint {
    (a * b) % unit
}

fn add_unit(a: &BigUint, b: &BigUint, unit: &BigUint) -> BigUint {
    (a + b) % unit
}

fn solve() -> String {
    let unit: BigUint = FromPrimitive::from_uint(100_0000_0000).unwrap();
    add_unit(
        &mul_unit(&FromPrimitive::from_uint(28433).unwrap(),
                  &pow_unit(&FromPrimitive::from_uint(2).unwrap(),
                            &FromPrimitive::from_uint(7830457).unwrap(),
                            &unit),
                  &unit),
        &One::one(),
        &unit
    ).to_string()
}

problem!("8739992577", solve);
