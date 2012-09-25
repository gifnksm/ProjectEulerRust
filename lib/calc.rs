pub fn each_fib(f: fn(uint)->bool) {
    let mut (prev, cur) = (0, 1);
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
        let fib = ~[ 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233 ];
        let mut calc = ~[];
        for each_fib |f| {
            if f > fib.last() { break; }
            calc += [ f ];
        }
        assert fib == calc;
    }
}
