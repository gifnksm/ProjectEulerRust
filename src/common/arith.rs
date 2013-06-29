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
