use num::{Integer, One, BigUint};

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

#[cfg(test)]
mod tests {
    use std::num::Int;

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
                    assert_eq!(b.pow(e) % r, super::mod_pow(b, e, r));
                }
            }
        }
    }
}
