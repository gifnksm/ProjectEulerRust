use core::cmp::{ Ord, Eq };
use core::hash::{ Hash };
use core::num::{ Zero, One };
use core::to_bytes::{ IterBytes };
use core::hashmap::linear::{ LinearMap };

pub fn each_triangles(f: &fn(uint) -> bool) {
    let mut idx = 0;
    let mut t   = 1;
    loop {
        if !f(t) { break; }
        idx += 1;
        t   += idx + 1;
    }
}

pub fn each_fib<T: One + Zero + Add<T, T>>(f: &fn(n: &T)->bool) {
    let mut (prev, cur) = (Zero::zero::<T>(), One::one::<T>());
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

pub fn histogram<T: Hash + IterBytes + Eq + Copy>(v: &[T]) -> LinearMap<T, uint> {
    let mut map = LinearMap::new::<T, uint>();
    for v.each |k| {
        let val = do map.find(k).map_default(1) |v| { *v + 1 };
        map.insert(*k, val);
    }
    return map;
}

pub fn num_of_permutations<T: Eq + Hash, M: Map<T, uint>>(hist: &M) -> uint {
    let mut sum = 0;
    let mut div = 1;
    for hist.each_value |cnt| {
        sum += *cnt;
        div *= factorial(*cnt);
    }
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

pub pure fn num_to_digits(n: uint, radix: uint) -> ~[uint] {
    let mut buf: [uint * 64] = [
               0, 0, 0, 0,  0, 0, 0, 0,
               0, 0, 0, 0,  0, 0, 0, 0,
               0, 0, 0, 0,  0, 0, 0, 0,
               0, 0, 0, 0,  0, 0, 0, 0,

               0, 0, 0, 0,  0, 0, 0, 0,
               0, 0, 0, 0,  0, 0, 0, 0,
               0, 0, 0, 0,  0, 0, 0, 0,
               0, 0, 0, 0,  0, 0, 0, 0
              ];
    let mut filled_idx = buf.len();
    let mut itr = n;
    while itr != 0 {
        buf[filled_idx - 1] = itr % radix;
        filled_idx -= 1;
        itr /= radix;
    }
    return vec::from_slice(buf.slice(filled_idx, buf.len()));
}

pub pure fn digits_to_num(v: &[uint], radix: uint) -> uint {
    let mut num = 0;
    for v.each |n| {
        num *= radix;
        num += *n;
    }
    return num;
}

pub pure fn combinate<T: Copy>(elems: &[T], len: uint, f: &fn(&[T], &[T])->bool) {
    if len == 0 {
        f(~[], elems);
        return;
    }

    for uint::range(0, elems.len() - len + 1) |i| {
        for combinate(elems.slice(i + 1, elems.len()), len - 1) |v, rest| {
            if !f(~[elems[i]] + v, ~[] + elems.slice(0, i) + rest) { return; }
        }
    }
}

pub pure fn combinate_overlap<T: Copy>(elems: &[T], len: uint, f: &fn(&[T])->bool) {
    if len == 0 {
        f(~[]);
        return;
    }

    for uint::range(0, elems.len()) |i| {
        for combinate_overlap(elems.slice(i, elems.len()), len - 1) |v| {
            if !f(~[elems[i]] + v) { return; }
        }
    }
}

pub pure fn permutate_num(digits: &[uint], len: uint, min: uint, max: uint,
                      f: &fn(uint, &[uint])->bool) {
    let min_vec = fill_zero(num_to_digits(min, 10), len);
    let max_vec = fill_zero(num_to_digits(max, 10), len);
    return perm_sub(digits, len, to_some(min_vec), to_some(max_vec), f);

    pure fn fill_zero(v: &[uint], n: uint) -> ~[uint] {
        fail_unless!(n >= v.len());
        vec::from_elem(n - v.len(), 0) + v
    }

    pure fn to_some(v: &'a [uint]) -> Option<&'a [uint]> { Some(v) }

    pure fn perm_sub(digits: &[uint], len: uint,
                     min: Option<&[uint]>,
                     max: Option<&[uint]>,
                     f: &fn(uint, &[uint])->bool) {
        if len == 0 {
            f(0, digits);
            return;
        }

        let unit = {
            let mut tmp = 1;
            for (len-1).times { tmp *= 10 }
            tmp
        };

        let mut buf = vec::from_elem(digits.len() - 1, 0);

        for digits.eachi |i, np| {
            let n = *np;

            let min_vec = match min {
                Some(v) if n <  v[0] => loop,
                Some(v) if n == v[0] => Some(vec::slice(v, 1, v.len())),
                _ => None
            };
            let max_vec = match max {
                Some(v) if n >  v[0] => loop,
                Some(v) if n == v[0] => Some(vec::slice(v, 1, v.len())),
                _ => None
            };

            for uint::range(0, i)         |j| { buf[j] = digits[j]; }
            for uint::range(i, buf.len()) |j| { buf[j] = digits[j + 1]; }
            for perm_sub(buf, len - 1, min_vec, max_vec) |num, ds| {
                if !f(num + n * unit, ds) { return; }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sort::{merge_sort};

    #[test]
    fn test_each_fib() {
        let fib = ~[ 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233 ];
        let mut calc = ~[];
        for each_fib |f: &uint| {
            if *f > *fib.last() { break; }
            calc += [ *f ];
        }
        fail_unless!(fib == calc);
    }

    #[test]
    fn test_factorial() {
        fail_unless!(factorial(0) == 1);
        fail_unless!(factorial(1) == 1);
        fail_unless!(factorial(2) == 2);
        fail_unless!(factorial(3) == 6);
        fail_unless!(factorial(10) == 3628800);
    }

    #[test]
    fn test_histogram() {
        fn check(inp: &[uint], result: &[(uint, uint)]) {
            let mut vec = ~[];
            for histogram(inp).each |&(&k, &v)| { vec += ~[(k, v)]; }
            let sorted = do merge_sort(vec) |a, b| { a.first() <= b.first() };
            fail_unless!(result == sorted);
        }
        check(&[1, 2, 3], &[(1, 1), (2, 1), (3, 1)]);
        check(&[1, 1, 1, 2, 2, 3, 3, 4], &[(1, 3), (2, 2), (3, 2), (4, 1)]);
        check(&[1, 1, 1, 2, 2, 1, 1], &[(1, 5), (2, 2)]);
        check(&[], &[]);
    }

    #[test]
    fn test_num_of_permutasions() {
        fail_unless!(num_of_permutations(histogram::<uint>(&[])) == 1);
        fail_unless!(num_of_permutations(histogram(&[1, 2, 3])) == 6);
        fail_unless!(num_of_permutations(histogram(&[1, 1, 1, 2, 3])) == 20);
        fail_unless!(num_of_permutations(histogram(&[1, 1, 1, 2, 3, 1, 1])) == 42);
    }

    #[test]
    fn test_get_gcd() {
        fail_unless!(get_gcd(2, 2) == 2);
        fail_unless!(get_gcd(100, 99) == 1);
        fail_unless!(get_gcd(8 * 3, 8 * 5) == 8);
    }

    #[test]
    fn test_num_to_digits() {
        fail_unless!(num_to_digits(0, 10) == ~[]);
        fail_unless!(num_to_digits(1, 10) == ~[1]);
        fail_unless!(num_to_digits(10, 10) == ~[1, 0]);
    }

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn test_num_to_digits_64() {
            fail_unless!(num_to_digits(-1, 10) == ~[1, 8, 4, 4, 6, 7, 4, 4, 0, 7, 3, 7, 0, 9, 5, 5, 1, 6, 1, 5]);
    }

    #[cfg(target_arch = "x86")]
    #[cfg(target_arch = "arm")]
    #[test]
    fn test_num_to_digits_32() {
            fail_unless!(num_to_digits(-1, 10) == ~[4, 2, 9, 4, 9, 6, 7, 2, 9, 5]);
    }

    #[test]
    fn test_digits_to_num() {
        fail_unless!(digits_to_num(~[], 10) == 0);
        fail_unless!(digits_to_num(~[1], 10) == 1);
        fail_unless!(digits_to_num(~[1, 2, 3], 10) == 123);
        fail_unless!(digits_to_num(~[0, 0, 1, 2, 3], 10) == 123);
        fail_unless!(digits_to_num(~[1, 2, 3, 0, 0], 10) == 12300);
    }

    #[test]
    fn test_combinate() {
        let mut nums = ~[~[1, 2, 3], ~[1, 2, 4], ~[1, 2, 5], ~[1, 3, 4], ~[1, 3, 5], ~[1, 4, 5],
                         ~[2, 3, 4], ~[2, 3, 5], ~[2, 4, 5], ~[3, 4, 5]];
        for combinate(&[1, 2, 3, 4, 5], 3) |n, _rest| {
            fail_unless!(n == vec::shift(&mut nums));
        }
    }

    #[test]
    fn test_combinate_overlap() {
        let mut nums = ~[~[1, 1, 1], ~[1, 1, 2], ~[1, 1, 3], ~[1, 1, 4], ~[1, 1, 5],
                         ~[1, 2, 2], ~[1, 2, 3], ~[1, 2, 4], ~[1, 2, 5],
                         ~[1, 3, 3], ~[1, 3, 4], ~[1, 3, 5],
                         ~[1, 4, 4], ~[1, 4, 5],
                         ~[1, 5, 5],
                         ~[2, 2, 2], ~[2, 2, 3], ~[2, 2, 4], ~[2, 2, 5],
                         ~[2, 3, 3], ~[2, 3, 4], ~[2, 3, 5],
                         ~[2, 4, 4], ~[2, 4, 5],
                         ~[2, 5, 5],
                         ~[3, 3, 3], ~[3, 3, 4], ~[3, 3, 5],
                         ~[3, 4, 4], ~[3, 4, 5],
                         ~[3, 5, 5],
                         ~[4, 4, 4], ~[4, 4, 5],
                         ~[4, 5, 5],
                         ~[5, 5, 5]];
        for combinate_overlap(&[1, 2, 3, 4, 5], 3) |n| {
            fail_unless!(n == vec::shift(&mut nums));
        }
    }

    #[test]
    fn test_permutate_num() {
        let mut nums = ~[ 123, 124, 125, 132, 134, 135, 142, 143, 145, 152, 153, 154,
                          213, 214, 215, 231, 234, 235, 241, 243, 245, 251, 253, 254,
                          312, 314, 315, 321, 324, 325, 341, 342, 345, 351, 352, 354,
                          412, 413, 415, 421, 423, 425, 431, 432, 435, 451, 452, 453,
                          512, 513, 514, 521, 523, 524, 531, 532, 534, 541, 542, 543];
        for permutate_num(&[1,2,3,4,5], 3, 0, 555) |n, _rest| {
            fail_unless!(n == vec::shift(&mut nums));
        }

        let mut nums = ~[ 123, 124, 125, 132, 134, 135, 142, 143, 145, 152, 153, 154,
                          213, 214, 215, 231, 234, 235, 241, 243, 245, 251, 253, 254,
                          312, 314, 315, 321, 324, 325, 341, 342, 345, 351, 352, 354,
                          412, 413, 415, 421, 423, 425, 431, 432, 435, 451, 452, 453,
                          512, 513, 514, 521, 523, 524, 531, 532, 534, 541, 542, 543];
        for permutate_num(&[1,2,3,4,5], 3, 140, 300) |n, _rest| {
            let mut num = vec::shift(&mut nums);
            while num < 140 || 300 < num {
                num = vec::shift(&mut nums);
            }
            fail_unless!(n == num);
        }
    }
}
