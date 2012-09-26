pub pure fn isqrt(n: uint) -> uint {
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
