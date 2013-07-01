use std::{uint, vec};
use std::num::{Zero, One};

fn omit_zeros<'a, T: Zero>(v: &'a [T]) -> &'a [T] {
    let len = v.rposition(|&n| !n.is_zero()).map_default(Zero::zero(), |&p| p + 1);
    return v.slice(0, len);
}

pub fn add<T: Zero + Add<T,T> + Clone>(a: &[T], b: &[T]) -> ~[T] {
    let a = omit_zeros(a);
    let b = omit_zeros(b);

    let (min_len, max_len, rest) = match a.len().cmp(&b.len()) {
        Less    => (a.len(), b.len(), Some(b)),
        Greater => (b.len(), a.len(), Some(a)),
        Equal   => (a.len(), a.len(), None)
    };

    let mut sum = vec::from_fn(max_len, |_i| Zero::zero());
    for uint::range(0, min_len) |i| { sum[i] = a[i] + b[i]; }
    do rest.map |&v| { for uint::range(min_len, max_len) |i| { sum[i] = v[i].clone(); } };
    return sum;
}

pub fn mul<T: Zero + Add<T, T> + Mul<T, T>>(a: &[T], b: &[T]) -> ~[T] {
    let a = omit_zeros(a);
    let b = omit_zeros(b);

    if a.is_empty() || b.is_empty() { return ~[]; }
    let mut prod = vec::from_fn(a.len() + b.len() - 1, |_i| Zero::zero::<T>());
    for a.iter().enumerate().advance |(i, &na)| {
        for b.iter().enumerate().advance |(j, &nb)| {
            prod[i + j] = prod[i + j] + na * nb;
        }
    }
    return prod;
}

pub fn eval<T: Zero + One + Add<T, T> + Mul<T, T>>(a: &[T], x: T) -> T {
    let mut sum = Zero::zero::<T>();
    let mut x_n = One::one();
    for uint::range(0, a.len()) |i| {
        sum = sum + a[i] * x_n;
        x_n = x_n * x;
    }
    return sum;
}

pub fn to_str<T: Zero + One + Eq + Neg<T> + ToStr + Ord>(a: &[T], x: &str) -> ~str {
    let a = omit_zeros(a);
    if a.is_empty() { return ~"0"; }

    let one = One::one();

    let mut s = ~[];
    for a.iter().enumerate().advance |(i, &n)| {
        // output n*x^i / -n*x^i
        if n.is_zero() { loop; }

        let term = if i.is_zero() {
            n.to_str()
        } else if i == 1 {
            if n == one { x.to_str() }
            else if n == -one { fmt!("-%s", x) }
            else { fmt!("%s*%s", n.to_str(), x) }
        } else {
            if n == one { fmt!("%s^%u", x, i) }
            else if n == -one { fmt!("-%s^%u", x, i) }
            else { fmt!("%s*%s^%u", n.to_str(), x, i) }
        };

        if s.len() > 0 && n > Zero::zero() { s.push(~"+"); }
        s.push(term);
    }

    return s.concat();
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{int};

    #[test]
    fn test_poly_add() {
        fn check(a: &[int], b: &[int], c: &[int]) {
            assert_eq!(add(a, b), c.to_owned());
            assert_eq!(add(b, a), c.to_owned());
        }
        check([], [], []);
        check([0, 0], [], []);
        check([0, 0], [1], [1]);
        check([1, 0], [1], [2]);
        check([1, 0, 1], [1], [2, 0, 1]);
    }

    #[test]
    fn test_poly_mul() {
        fn check(a: &[int], b: &[int], c: &[int]) {
            assert_eq!(mul(a, b), c.to_owned());
            assert_eq!(mul(b, a), c.to_owned());
        }
        check([], [], []);
        check([0, 0], [], []);
        check([0, 0], [1], []);
        check([1, 0], [1], [1]);
        check([1, 0, 1], [1], [1, 0, 1]);
        check([1, 1], [1, 1], [1, 2, 1]);
        check([1, 1], [1, 0, 1], [1, 1, 1, 1]);
        check([0, 0, 1], [0, 0, 1], [0, 0, 0, 0, 1]);
    }

    #[test]
    fn test_poly_eaval() {
        fn check(pol: &[int], f: &fn(int) -> int) {
            for int::range(-10, 10) |n| {
                assert_eq!(eval(pol, n), f(n));
            }
        }
        check([], |_x| 0);
        check([1], |_x| 1);
        check([1, 1], |x| x + 1);
        check([0, 1], |x| x);
        check([10, -10, 10], |x| 10*x*x - 10 * x + 10);
    }

    #[test]
    fn test_poly_to_str() {
        assert_eq!(to_str::<int>([], "x"), ~"0");
        assert_eq!(to_str([1], "x"), ~"1");
        assert_eq!(to_str([1, 1], "x"), ~"1+x");
        assert_eq!(to_str([1, 1, 1], "x"), ~"1+x+x^2");
        assert_eq!(to_str([2, 2, 2], "x"), ~"2+2*x+2*x^2");
        assert_eq!(to_str([0, 0, 0, 1], "x"), ~"x^3");
        assert_eq!(to_str([0, 0, 0, -1], "x"), ~"-x^3");
        assert_eq!(to_str([-1, 0, 0, -1], "x"), ~"-1-x^3");
        assert_eq!(to_str([-1, 1, 0, -1], "x"), ~"-1+x-x^3");
        assert_eq!(to_str([-1, 1, -1, -1], "x"), ~"-1+x-x^2-x^3");
    }
}
