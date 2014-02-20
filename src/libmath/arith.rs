use std::num::One;
use num::bigint::BigUint;

pub fn isqrt(n: uint) -> uint {
    let mut min = 0;
    let mut max = n;

    while min < max {
        let mid = (min + max + 1) / 2;
        if (mid * mid) == n {
            return mid;
        }

        if (mid * mid) >= n {
            max = mid - 1;
        } else {
            min = mid;
        }
    }

    return min;
}

pub fn mod_pow(base: uint, mut exp: uint, modulo: uint) -> uint {
    if base == 0 { return 0 }

    let mut result: BigUint = One::one();
    let mut base:   BigUint = FromPrimitive::from_uint(base).unwrap();
    let modulo: BigUint   = FromPrimitive::from_uint(modulo).unwrap();

    while exp > 0 {
        if exp.is_odd() {
            result = (result * base) % modulo;
        }
        exp >>= 1;
        base = (base * base) % modulo;
    }
    result.to_uint().unwrap()
}

///
/// Divide two numbers, return the result, rounded up.
///
/// # Arguments
///
/// * x - an integer
/// * y - an integer distinct from 0u
///
/// # Return value
///
/// The smallest integer `q` such that `x/y <= q`.
///
pub fn div_ceil(x: uint, y: uint) -> uint {
    let div = x / y;
    if x % y == 0u { div }
    else { div + 1u }
}

///
/// Divide two numbers, return the result, rounded to the closest integer.
///
/// # Arguments
///
/// * x - an integer
/// * y - an integer distinct from 0u
///
/// # Return value
///
/// The integer `q` closest to `x/y`.
///
pub fn div_round(x: uint, y: uint) -> uint {
    let div = x / y;
    if x % y * 2u  < y { div }
    else { div + 1u }
}

///
/// Divide two numbers, return the result, rounded down.
///
/// Note: This is the same function as `div`.
///
/// # Arguments
///
/// * x - an integer
/// * y - an integer distinct from 0u
///
/// # Return value
///
/// The smallest integer `q` such that `x/y <= q`. This
/// is either `x/y` or `x/y + 1`.
///
pub fn div_floor(x: uint, y: uint) -> uint { return x / y; }

#[cfg(test)]
mod tests {
    use std::num;

    #[test]
    fn isqrt() {
        for x in range(0u, 10) {
            for x2 in range(x * x, (x + 1) * (x + 1)) {
                assert_eq!(super::isqrt(x2), x);
            }
        }
    }

    #[test]
    fn mod_pow() {
        for b in range(1u, 10) {
            for e in range(0u, 5) {
                for r in range(10u, 100) {
                    assert_eq!(num::pow(b, e) % r, super::mod_pow(b, e, r));
                }
            }
        }
    }

    #[test]
    fn div() {
        assert!((super::div_floor(3u, 4u) == 0u));
        assert!((super::div_ceil(3u, 4u)  == 1u));
        assert!((super::div_round(3u, 4u) == 1u));
    }
}
