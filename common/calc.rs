use core::cmp::{ Eq };
use core::hash::{ Hash };
use core::num::{ Zero, One, IntConvertible };
use core::to_bytes::{ IterBytes };
use core::hashmap::linear::{ LinearMap, LinearSet };
use core::util::{ swap };

use common::arith::{ isqrt };

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

pub fn digit_histogram(n: uint) -> [uint, ..10] {
    let mut hist = [0, ..10];
    let mut itr = n;
    while itr > 0 {
        hist[itr % 10] += 1;
        itr /= 10;
    }
    return hist;
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

pub fn num_to_digits(n: uint, radix: uint) -> ~[uint] {
    let mut buf: [uint, ..64] = [0, ..64];
    let mut filled_idx = buf.len();
    let mut itr = n;
    while itr != 0 {
        buf[filled_idx - 1] = itr % radix;
        filled_idx -= 1;
        itr /= radix;
    }
    return vec::from_slice(buf.slice(filled_idx, buf.len()));
}

pub fn digits_to_num(v: &[uint], radix: uint) -> uint {
    let mut num = 0;
    for v.each |n| {
        num *= radix;
        num += *n;
    }
    return num;
}

pub fn combinate<T: Copy>(elems: &[T], len: uint, f: &fn(&[T], &[T])->bool) {
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

pub fn combinate_overlap<T: Copy>(elems: &[T], len: uint, f: &fn(&[T])->bool) {
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

pub fn permutate_num(digits: &[uint], len: uint, min: uint, max: uint,
                      f: &fn(uint, &[uint])->bool) {
    let min_vec = fill_zero(num_to_digits(min, 10), len);
    let max_vec = fill_zero(num_to_digits(max, 10), len);
    return perm_sub(digits, len, to_some(min_vec), to_some(max_vec), f);

    fn fill_zero(v: &[uint], n: uint) -> ~[uint] {
        assert!(n >= v.len());
        vec::from_elem(n - v.len(), 0) + v
    }

    fn to_some<'a>(v: &'a [uint]) -> Option<&'a [uint]> { Some(v) }

    fn perm_sub(digits: &[uint], len: uint,
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

pub fn cont_frac_sqrt(n: uint) -> (uint, ~[uint]) {
    let mut a0 = 0;
    let mut an = ~[];
    let mut set = LinearSet::new();

    for each_a(n) |a, pqr| {
        if a == 0 || set.contains(&(a, pqr)) {
            break;
        }

        set.insert((a, pqr));
        if set.len() == 1 {
            a0 = a;
        } else {
            an.push(a);
        }
    }
    return (a0, an);

    // f_n (p, q, r) := (p sqrt(n) + q)/ r
    //                = a + (1 / (rp sqrt(n) + rb) / (np^2 - b^2))
    // a := |f_n(p, q, r)|
    // b := ar - q
    // (p, q, r) := (rp / m, rb / m, (np^2 - b^2) / m)
    #[inline(always)]
    fn each_a(n: uint, f: &fn(uint, (uint, uint, uint)) -> bool) {
        let sqn = isqrt(n);
        let mut (p, q, r) = (1, 0, 1);
        loop {
            let a = calc_a(n, sqn, (p, q, r));
            if a * a == n || p == 0 {
                p = 0; q = 0; r = 1;
            } else {
                let b = a * r - q;
                let (p2, q2, r2) = (r*p, r*b, n*p*p - b*b);
                let m = get_gcd(get_gcd(p2, q2), r2);
                p = p2 / m;
                q = q2 / m;
                r = r2 / m;
            }
            if !f(a, (p, q, r)) { break; }
        }
    }


    // a <= f_n(p, q, r) < a + 1
    // r a - q <= p sqrt(n) < r (a + 1) - pq
    // (ar - q)^2 <= np^2 < ((a+1)r - q)^2
    #[inline(always)]
    fn calc_a(n: uint, sqn: uint, (p, q, r): (uint, uint, uint)) -> uint {
        // g(a, r, q) := (ar - q)^2
        #[inline(always)]
        fn g(a: uint, r: uint, q: uint) -> uint {
            let s = a * r - q;
            return s * s;
        }

        let np2 = n * p * p;
        let estim_a = (p * sqn + q) / r;
        let mut a = estim_a;
        while g(a + 1, r, q) <= np2 {
            a = a + 1;
        }
        return a;
    }
}

pub fn fold_cont_frac<
    T: IntConvertible + Add<T, T> + Mul<T, T>
    >(an: &[uint]) -> (T, T) {
    let mut numer = IntConvertible::from_int::<T>(1);
    let mut denom = IntConvertible::from_int::<T>(0);

    for an.each_reverse |&a| {
        swap(&mut numer, &mut denom);
        numer = numer + IntConvertible::from_int::<T>(a as int) * denom;
    }

    return (numer, denom);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sort::{ merge_sort };
    use std::bigint::{ BigUint };

    #[test]
    fn test_each_fib() {
        let fib = ~[ 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233 ];
        let mut calc = ~[];
        for each_fib |f: &uint| {
            if *f > *fib.last() { break; }
            calc += [ *f ];
        }
        assert_eq!(fib, calc);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(10), 3628800);
    }

    #[test]
    fn test_histogram() {
        fn check(inp: &[uint], result: &[(uint, uint)]) {
            let vec = do merge_sort(iter::to_vec(&histogram(inp))) |a, b| {
                a.first() <= b.first()
            }.map(|&(&a, &b)| (a, b));
            assert_eq!(vec.initn(0), result);
        }
        check(&[1, 2, 3], ~[(1, 1), (2, 1), (3, 1)]);
        check(&[1, 1, 1, 2, 2, 3, 3, 4], ~[(1, 3), (2, 2), (3, 2), (4, 1)]);
        check(&[1, 1, 1, 2, 2, 1, 1], ~[(1, 5), (2, 2)]);
        check(&[], ~[]);
    }

    #[test]
    fn test_num_of_permutasions() {
        assert_eq!(num_of_permutations(&histogram::<uint>(&[])), 1);
        assert_eq!(num_of_permutations(&histogram(&[1, 2, 3])), 6);
        assert_eq!(num_of_permutations(&histogram(&[1, 1, 1, 2, 3])), 20);
        assert_eq!(num_of_permutations(&histogram(&[1, 1, 1, 2, 3, 1, 1])), 42);
    }

    #[test]
    fn test_get_gcd() {
        assert_eq!(get_gcd(2, 2), 2);
        assert_eq!(get_gcd(100, 99), 1);
        assert_eq!(get_gcd(8 * 3, 8 * 5), 8);
    }

    #[test]
    fn test_num_to_digits() {
        assert_eq!(num_to_digits(0, 10), ~[]);
        assert_eq!(num_to_digits(1, 10), ~[1]);
        assert_eq!(num_to_digits(10, 10), ~[1, 0]);
    }

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn test_num_to_digits_64() {
        assert_eq!(
            num_to_digits(-1, 10),
            ~[1, 8, 4, 4, 6, 7, 4, 4, 0, 7, 3, 7, 0, 9, 5, 5, 1, 6, 1, 5]);
    }

    #[cfg(target_arch = "x86")]
    #[cfg(target_arch = "arm")]
    #[test]
    fn test_num_to_digits_32() {
        assert_eq!(num_to_digits(-1, 10), ~[4, 2, 9, 4, 9, 6, 7, 2, 9, 5]);
    }

    #[test]
    fn test_digits_to_num() {
        assert_eq!(digits_to_num(~[], 10), 0);
        assert_eq!(digits_to_num(~[1], 10), 1);
        assert_eq!(digits_to_num(~[1, 2, 3], 10), 123);
        assert_eq!(digits_to_num(~[0, 0, 1, 2, 3], 10), 123);
        assert_eq!(digits_to_num(~[1, 2, 3, 0, 0], 10), 12300);
    }

    #[test]
    fn test_combinate() {
        let mut nums = ~[
            &[1, 2, 3], &[1, 2, 4], &[1, 2, 5], &[1, 3, 4], &[1, 3, 5], &[1, 4, 5],
            &[2, 3, 4], &[2, 3, 5], &[2, 4, 5],
            &[3, 4, 5]
        ];
        for combinate(&[1, 2, 3, 4, 5], 3) |n, _rest| {
            assert_eq!(n, vec::shift(&mut nums));
        }
    }

    #[test]
    fn test_combinate_overlap() {
        let mut nums = ~[
            &[1, 1, 1], &[1, 1, 2], &[1, 1, 3], &[1, 1, 4], &[1, 1, 5],
            &[1, 2, 2], &[1, 2, 3], &[1, 2, 4], &[1, 2, 5],
            &[1, 3, 3], &[1, 3, 4], &[1, 3, 5],
            &[1, 4, 4], &[1, 4, 5],
            &[1, 5, 5],
            &[2, 2, 2], &[2, 2, 3], &[2, 2, 4], &[2, 2, 5],
            &[2, 3, 3], &[2, 3, 4], &[2, 3, 5],
            &[2, 4, 4], &[2, 4, 5],
            &[2, 5, 5],
            &[3, 3, 3], &[3, 3, 4], &[3, 3, 5],
            &[3, 4, 4], &[3, 4, 5],
            &[3, 5, 5],
            &[4, 4, 4], &[4, 4, 5],
            &[4, 5, 5],
            &[5, 5, 5]
        ];

        for combinate_overlap(&[1, 2, 3, 4, 5], 3) |n| {
            assert_eq!(n, vec::shift(&mut nums));
        }
    }

    #[test]
    fn test_permutate_num() {
        let mut nums = ~[
            123, 124, 125, 132, 134, 135, 142, 143, 145, 152, 153, 154,
            213, 214, 215, 231, 234, 235, 241, 243, 245, 251, 253, 254,
            312, 314, 315, 321, 324, 325, 341, 342, 345, 351, 352, 354,
            412, 413, 415, 421, 423, 425, 431, 432, 435, 451, 452, 453,
            512, 513, 514, 521, 523, 524, 531, 532, 534, 541, 542, 543
        ];

        for permutate_num(&[1, 2, 3, 4, 5], 3, 0, 555) |n, _rest| {
            assert_eq!(n, vec::shift(&mut nums));
        }

        let mut nums = ~[
            123, 124, 125, 132, 134, 135, 142, 143, 145, 152, 153, 154,
            213, 214, 215, 231, 234, 235, 241, 243, 245, 251, 253, 254,
            312, 314, 315, 321, 324, 325, 341, 342, 345, 351, 352, 354,
            412, 413, 415, 421, 423, 425, 431, 432, 435, 451, 452, 453,
            512, 513, 514, 521, 523, 524, 531, 532, 534, 541, 542, 543
        ];

        for permutate_num(&[1, 2, 3, 4, 5], 3, 140, 300) |n, _rest| {
            let mut num = vec::shift(&mut nums);
            while num < 140 || 300 < num {
                num = vec::shift(&mut nums);
            }
            assert_eq!(n, num);
        }
    }

    #[test]
    fn test_cont_frac_sqrt() {
        assert_eq!(cont_frac_sqrt(1), (1, ~[]));
        assert_eq!(cont_frac_sqrt(2), (1, ~[2]));
        assert_eq!(cont_frac_sqrt(3), (1, ~[1,2]));
        assert_eq!(cont_frac_sqrt(4), (2, ~[]));
        assert_eq!(cont_frac_sqrt(5), (2, ~[4]));
        assert_eq!(cont_frac_sqrt(6), (2, ~[2,4]));
        assert_eq!(cont_frac_sqrt(7), (2, ~[1,1,1,4]));
        assert_eq!(cont_frac_sqrt(8), (2, ~[1,4]));
        assert_eq!(cont_frac_sqrt(9), (3, ~[]));
        assert_eq!(cont_frac_sqrt(10), (3, ~[6]));
        assert_eq!(cont_frac_sqrt(11), (3, ~[3,6]));
        assert_eq!(cont_frac_sqrt(12), (3, ~[2,6]));
        assert_eq!(cont_frac_sqrt(13), (3, ~[1,1,1,1,6]));
    }

    #[test]
    fn test_fold_cont_frac() {
        fn test(an: &[uint], (n, d): (uint, uint)) {
            assert_eq!(
                fold_cont_frac::<BigUint>(an),
                (BigUint::from_uint(n), BigUint::from_uint(d))
            );
        }
        test(~[1, 2], (3, 2));
        test(~[1, 2, 2], (7, 5));
        test(~[1, 2, 2, 2], (17, 12));
        test(~[1, 2, 2, 2, 2], (41, 29));

        test(~[2], (2, 1));
        test(~[2, 1], (3, 1));
        test(~[2, 1, 2], (8, 3));
        test(~[2, 1, 2, 1], (11, 4));
        test(~[2, 1, 2, 1, 1], (19, 7));
        test(~[2, 1, 2, 1, 1, 4], (87, 32));
        test(~[2, 1, 2, 1, 1, 4, 1], (106, 39));
        test(~[2, 1, 2, 1, 1, 4, 1, 1], (193, 71));
        test(~[2, 1, 2, 1, 1, 4, 1, 1, 6], (1264, 465));
        test(~[2, 1, 2, 1, 1, 4, 1, 1, 6, 1], (1457, 536));
    }
}
