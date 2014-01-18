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
    #[test]
    fn test_isqrt() {
        for x in range(0u, 10) {
            for x2 in range(x * x, (x + 1) * (x + 1)) {
                assert_eq!(super::isqrt(x2), x);
            }
        }
    }
}
