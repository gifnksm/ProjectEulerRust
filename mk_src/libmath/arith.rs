use num::{Integer, One, BigUint};

pub fn mod_pow(base: uint, mut exp: uint, modulo: uint) -> uint {
    if base == 0 { return 0 }

    let mut result: BigUint = One::one();
    let mut base:   BigUint = FromPrimitive::from_uint(base).unwrap();
    let modulo: BigUint   = FromPrimitive::from_uint(modulo).unwrap();

    while exp > 0 {
        if exp.is_odd() {
            result = (result * &base) % &modulo;
        }
        exp >>= 1;
        base = (&base * &base) % &modulo;
    }
    result.to_uint().unwrap()
}

#[cfg(test)]
mod tests {
    use std::num::Int;

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
