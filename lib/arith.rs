pure fn isqrt(n: u64) -> u64 {
    let mut (min, max) = (0, n);
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

pure fn div_rem(n: uint, d: uint) -> (uint, uint) {
    (n / d, n % d)
}

pure fn add_carry(n1: uint, n2: uint) -> (uint, uint) {
    add_carry2((n1, 0), n2)
}

pure fn add_carry2(car: (uint, uint), n2: uint) -> (uint, uint) {
    let (n1, prev_carry) = car;
    let carry = if n1 > uint::max_value - n2 { 1 } else { 0 };
    if carry > uint::max_value - prev_carry { fail ~"carry overflow" }
    (n1 + n2, carry + prev_carry)
}

pure fn add_carry_multi(ns: &[uint]) -> (uint, uint) {
    ns.foldl((0, 0), |accum, n| add_carry2(accum, n))
}

pure fn sub_borrow(n1: uint, n2: uint) -> (uint, uint) {
    sub_borrow2((n1, 0), n2)
}

pure fn sub_borrow2(bor: (uint, uint), n2: uint) -> (uint, uint) {
    let (n1, prev_borrow) = bor;
    let borrow = if n1 - uint::min_value < n2 { 1u } else { 0u };
    (n1 - n2, prev_borrow + borrow)
}

pure fn sub_borrow_multi(n0: uint, ns: &[uint]) -> (uint, uint) {
    ns.foldl((n0, 0), |accum, n| sub_borrow2(accum, n))
}

pure fn mul_carry(n1: uint, n2: uint) -> (uint, uint) {
    let halfbits = sys::size_of::<uint>() * 8u / 2u;
    let div      = (1u << halfbits);

    let (hi1, lo1) = div_rem(n1, div);
    let (hi2, lo2) = div_rem(n2, div);
    let (mid12_hi, mid12_lo) = div_rem(lo1 * hi2, div);
    let (mid21_hi, mid21_lo) = div_rem(hi1 * lo2, div);
    let (lo_sum, lo_carry) = add_carry_multi([lo1 * lo2, mid12_lo * div, mid21_lo * div]);
    let (hi_sum, hi_carry) = add_carry_multi([hi1 * hi2, mid12_hi, mid21_hi, lo_carry]);
    assert hi_carry == 0u;
    return (lo_sum, hi_sum);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_div_rem() {
        let (d, m) = div_rem(1234u, 56u);
        assert d * 56u + m == 1234u;
    }

    #[test]
    fn test_add_carry() {
        fn check_add(n1: uint, n2: uint, sum: (uint, uint)) {
            assert add_carry(n1, n2) == sum;
            assert add_carry(n2, n1) == sum;
        }
        check_add(0u, 0u, (0u, 0u));
        check_add(0u, uint::max_value, (uint::max_value, 0u));
        check_add(1u, 3u, (1u + 3u, 0u));
        check_add(uint::max_value / 2u, uint::max_value / 2u, (uint::max_value - 1u, 0u));
        check_add(uint::max_value / 2u + 1u, uint::max_value / 2u, (uint::max_value, 0u));
        check_add(uint::max_value / 2u + 1u, uint::max_value / 2u + 1u, (0u, 1u));
        check_add(1000u, uint::max_value, (999u, 1u));
    }

    #[test]
    fn test_add_carry_multi() {
        assert add_carry_multi([0u, 0u, 0u]) == (0u, 0u);
        assert add_carry_multi([uint::max_value, uint::max_value, uint::max_value]) == (uint::max_value - 2u, 2u);
        assert add_carry_multi([uint::max_value, uint::max_value, uint::max_value, 2u]) == (uint::max_value, 2u);
        assert add_carry_multi([uint::max_value, uint::max_value, uint::max_value, 3u]) == (0u, 3u);
    }

    #[test]
    fn test_sub_borrow() {
        assert sub_borrow(0u, 0u) == (0u, 0u);
        assert sub_borrow(0u, 100u) == (uint::max_value - 99u, 1u);
        assert sub_borrow(100u, 100u) == (0u, 0u);
        assert sub_borrow(100u, 50u) == (50u, 0u);
        assert sub_borrow(100u, 200u) == (uint::max_value - 99u, 1u);
    }

    #[test]
    fn test_sub_borrow_multi() {
        assert sub_borrow_multi(0u, [0u, 0u, 0u, 0u]) == (0u, 0u);
        assert sub_borrow_multi(5u, [1u, 1u, 1u, 1u]) == (1u, 0u);
        assert sub_borrow_multi(0u, [1u, 1u, 1u, 1u]) == (uint::max_value - 3u, 1u);
        assert sub_borrow_multi(5u, [2u, 2u, 2u, 2u]) == (uint::max_value - 2u, 1u);
        assert sub_borrow_multi(0u, [uint::max_value, uint::max_value, uint::max_value, uint::max_value]) == (4u, 4u);
    }

    #[test]
    fn test_mul_carry() {
        fn check_mul(n1: uint, n2: uint, mul: (uint, uint)) {
            assert mul_carry(n1, n2) == mul;
            assert mul_carry(n2, n1) == mul;
        }

        check_mul(2u, 6u, (12u, 0u));

        fn check_mul_half(n: uint) {
            if n % 2u == 0u {
                // (2^(n-1) - 1) (2a)     = 2^n (a - 1) + (2^n - 1) - 2a + 1
                check_mul(uint::max_value / 2u, n, (uint::max_value - n + 1u, n / 2u - 1u));
            } else {
                // (2^(n-1) - 1) (2a + 1) = 2^n a + (2^(n-1) - 1) - 2a
                check_mul(uint::max_value / 2u, n, (uint::max_value / 2u - (n - 1u), n / 2u));
            }
        }
        check_mul_half(1u);
        check_mul_half(2u);
        check_mul_half(3u);
        check_mul_half(100u);

        // (2^n-1) a = 2^n(a - 1) + (2^n-1) - a + 1
        fn check_mul_full(n: uint) {
            check_mul(uint::max_value, n, (uint::max_value - n + 1u, n - 1u));
        }
        check_mul_full(1u);
        check_mul_full(2u);
        check_mul_full(3u);
        check_mul_full(0xFFFFFFFFu);
        check_mul_full(uint::max_value / 2u);
        check_mul_full(uint::max_value);
    }
}
