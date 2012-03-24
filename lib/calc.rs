fn isqrt(n: u64) -> u64 {
    let mut (min, max) = (0u64, n);
    while min < max {
        let mid = (min + max + 1u64) / 2u64;
        if (mid * mid) == n {
            ret mid;
        } else if (mid * mid) >= n {
            max = mid - 1u64;
        } else {
            min = mid;
        }
    }
    ret min;
}

