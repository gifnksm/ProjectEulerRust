pub fn pow(base: uint, exp: uint) -> uint {
    let mut result = 1;
    let mut itr = exp;
    let mut pow = base;
    while itr > 0 {
        if itr & 0x1 == 0x1 {
            result *= pow;
        }
        itr >>= 1;
        pow *= pow;
    }
    return result;
}

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pow() {
        assert_eq!(pow(0, 0), 1);
        assert_eq!(pow(0, 1), 0);
        assert_eq!(pow(1, 1), 1);
        assert_eq!(pow(1, 100), 1);

        assert_eq!(pow(2, 0), 1);
        assert_eq!(pow(2, 1), 2);
        assert_eq!(pow(2, 2), 4);
        assert_eq!(pow(2, 10), 1024);

        assert_eq!(pow(3, 0), 1);
        assert_eq!(pow(3, 1), 3);
    }

    #[test]
    fn test_isqrt() {
        for x in range(0u, 10) {
            for x2 in range(x * x, (x + 1) * (x + 1)) {
                assert_eq!(isqrt(x2), x);
            }
        }
    }
}
