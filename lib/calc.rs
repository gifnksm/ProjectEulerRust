use dvec::{ DVec };
use cmp::{ Ord, Eq };
use to_bytes::{ IterBytes };
use hash::{ Hash };
use num::{ from_int };
use std::map::{ HashMap };

pub fn each_fib<T: Num Copy>(f: fn(n: &T)->bool) {
    let mut (prev, cur) = (from_int::<T>(0), from_int::<T>(1));
    loop {
        if !f(&cur) { break; }
        let next = prev + cur;
        prev = cur;
        cur  = next;
    }
}

pub fn factorial(n: uint) -> uint {
    let mut prod = 1;
    for uint::range(1, n + 1) |n| { prod *= n; }
    return prod;
}

pub fn histogram<T: Eq IterBytes Hash Const Copy>(v: &[T]) -> HashMap<T, uint> {
    let map = HashMap::<T, uint>();
    for v.each |k| {
        let val = do map.find(*k).map_default(1) |v| { *v + 1 };
        map.insert(*k, val);
    }
    return map;
}

pub fn num_of_permutations<T: Eq IterBytes Hash Copy>(hist: HashMap<T, uint>) -> uint {
    let mut sum = 0;
    let mut div = 1;
    for hist.each_value |cnt| { sum += cnt; div *= factorial(cnt); }
    return factorial(sum) / div;
}

pub fn get_gcd(a: uint, b: uint) -> uint {
    let mut p = uint::max(a, b);
    let mut q = uint::min(a, b);
    loop {
        let mut r = p % q;
        if r == 0 { return q; }
        p = q;
        q = r;
    }
}

pub pure fn num_to_digits(n: uint) -> ~[uint] {
    let buf = [mut
               0, 0, 0, 0,  0, 0, 0, 0,
               0, 0, 0, 0,  0, 0, 0, 0,
               0, 0, 0, 0,  0, 0, 0, 0,
               0, 0, 0, 0,  0, 0, 0, 0,

               0, 0, 0, 0,  0, 0, 0, 0,
               0, 0, 0, 0,  0, 0, 0, 0,
               0, 0, 0, 0,  0, 0, 0, 0,
               0, 0, 0, 0,  0, 0, 0, 0
              ]/64;
    let mut filled_idx = buf.len();
    let mut itr = n;
    while itr != 0 {
        buf[filled_idx - 1] = itr % 10;
        filled_idx -= 1;
        itr /= 10;
    }
    return vec::from_slice(vec::view(buf, filled_idx, buf.len()));
}

pub pure fn digits_to_num(v: &[uint]) -> uint {
    let mut num = 0;
    for v.each |n| {
        num *= 10;
        num += *n;
    }
    return num;
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_each_fib() {
        let fib = ~[ 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233 ];
        let mut calc = ~[];
        for each_fib |f: &uint| {
            if *f > fib.last() { break; }
            calc += [ *f ];
        }
        assert fib == calc;
    }

    #[test]
    fn test_factorial() {
        assert factorial(0) == 1;
        assert factorial(1) == 1;
        assert factorial(2) == 2;
        assert factorial(3) == 6;
        assert factorial(10) == 3628800;
    }

    #[test]
    fn test_histogram() {
        fn check(inp: &[uint], result: &[(uint, uint)]) {
            let mut vec = ~[];
            for histogram(inp).each |k, v| { vec += [(k, v)]; }
            let sorted = std::sort::merge_sort(|a, b| a.first() <= b.first(), vec);
            assert result == sorted;
        }
        check(&[1, 2, 3], &[(1, 1), (2, 1), (3, 1)]);
        check(&[1, 1, 1, 2, 2, 3, 3, 4], &[(1, 3), (2, 2), (3, 2), (4, 1)]);
        check(&[1, 1, 1, 2, 2, 1, 1], &[(1, 5), (2, 2)]);
        check(&[], &[]);
    }

    #[test]
    fn test_num_of_permutasions() {
        assert num_of_permutations(histogram::<uint>(&[])) == 1;
        assert num_of_permutations(histogram(&[1, 2, 3])) == 6;
        assert num_of_permutations(histogram(&[1, 1, 1, 2, 3])) == 20;
        assert num_of_permutations(histogram(&[1, 1, 1, 2, 3, 1, 1])) == 42;
    }

    #[test]
    fn test_get_gcd() {
        assert get_gcd(2, 2) == 2;
        assert get_gcd(100, 99) == 1;
        assert get_gcd(8 * 3, 8 * 5) == 8;
    }

    #[test]
    fn test_num_to_digits() {
        assert num_to_digits(0) == ~[];
        assert num_to_digits(1) == ~[1];
        assert num_to_digits(10) == ~[1, 0];
    }

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn test_num_to_digits_64() {
            assert num_to_digits(-1) == ~[1, 8, 4, 4, 6, 7, 4, 4, 0, 7, 3, 7, 0, 9, 5, 5, 1, 6, 1, 5];
    }

    #[cfg(target_arch = "x86")]
    #[cfg(target_arch = "arm")]
    #[test]
    fn test_num_to_digits_32() {
            assert num_to_digits(-1) == ~[4, 2, 9, 4, 9, 6, 7, 2, 9, 5];
    }

    #[test]
    fn test_digits_to_num() {
        assert digits_to_num(~[]) == 0;
        assert digits_to_num(~[1]) == 1;
        assert digits_to_num(~[1, 2, 3]) == 123;
        assert digits_to_num(~[0, 0, 1, 2, 3]) == 123;
        assert digits_to_num(~[1, 2, 3, 0, 0]) == 12300;
    }
}
