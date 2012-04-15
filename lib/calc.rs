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

fn each_fib(f: fn(&&u64)->bool) {
    let mut (prev, cur) = (0u64, 1u64);
    loop {
        if !f(cur) { break; }
        let next = prev + cur;
        prev = cur;
        cur  = next;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_each_fib() {
        let fib = [ 1u64, 1u64, 2u64, 3u64, 5u64, 8u64, 13u64, 21u64, 34u64, 55u64, 89u64, 144u64, 233u64 ];
        let mut calc = [];
        for each_fib {|f|
            if f > fib.last() { break; }
            calc += [ f ];
        };
        assert fib == calc;
    }
}
