//! [Problem 97](https://projecteuler.net/problem=97) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use num_bigint::BigUint;
use num_traits::{FromPrimitive, One, Zero};

fn pow_unit(base: &BigUint, exp: &BigUint, unit: &BigUint) -> BigUint {
    let two: BigUint = FromPrimitive::from_u32(2).unwrap();
    let mut result = One::one();
    let mut itr = exp.clone();
    let mut pow = base.clone();
    while !itr.is_zero() {
        if &itr % &two == One::one() {
            result = mul_unit(&result, &pow, unit);
        }
        itr >>= 1;
        pow = mul_unit(&pow, &pow, unit);
    }
    result
}

fn mul_unit(a: &BigUint, b: &BigUint, unit: &BigUint) -> BigUint {
    (a * b) % unit
}

fn add_unit(a: &BigUint, b: &BigUint, unit: &BigUint) -> BigUint {
    (a + b) % unit
}

fn solve() -> String {
    let unit: BigUint = FromPrimitive::from_u64(100_0000_0000).unwrap();
    add_unit(
        &mul_unit(
            &FromPrimitive::from_u32(28433).unwrap(),
            &pow_unit(
                &FromPrimitive::from_u32(2).unwrap(),
                &FromPrimitive::from_u32(7830457).unwrap(),
                &unit,
            ),
            &unit,
        ),
        &One::one(),
        &unit,
    )
    .to_string()
}

common::problem!("8739992577", solve);
