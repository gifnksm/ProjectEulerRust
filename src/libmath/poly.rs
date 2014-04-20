use std::num::{Zero, One};

fn omit_zeros<'a, T: Zero>(v: &'a [T]) -> &'a [T] {
    let len = v.iter().rposition(|n| !n.is_zero()).map_or(0, |p| p + 1);
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

    let mut sum = Vec::from_fn(max_len, |_i| Zero::zero());
    for i in range(0, min_len) { *sum.get_mut(i) = a[i] + b[i]; }
    rest.map(|v| { for i in range(min_len, max_len) { *sum.get_mut(i) = v[i].clone(); } });
    sum.as_slice().to_owned()
}

pub fn mul<T: Zero + Add<T, T> + Mul<T, T>>(a: &[T], b: &[T]) -> ~[T] {
    let a = omit_zeros(a);
    let b = omit_zeros(b);

    if a.is_empty() || b.is_empty() { return ~[]; }
    let mut prod: Vec<T> = Vec::from_fn(a.len() + b.len() - 1, |_i| Zero::zero());
    for (i, na) in a.iter().enumerate() {
        for (j, nb) in b.iter().enumerate() {
            *prod.get_mut(i + j) = *prod.get(i + j) + (*na) * (*nb);
        }
    }
    prod.move_iter().collect()
}

pub fn eval<T: Zero + One + Add<T, T> + Mul<T, T>>(a: &[T], x: T) -> T {
    let mut sum: T = Zero::zero();
    let mut x_n = One::one();
    for i in range(0, a.len()) {
        sum = sum + a[i] * x_n;
        x_n = x_n * x;
    }
    return sum;
}

pub fn to_str<T: Zero + One + Eq + Neg<T> + ToStr + Ord>(a: &[T], x: &str) -> ~str {
    let a = omit_zeros(a);
    if a.is_empty() { return ~"0"; }

    let one = One::one();

    let mut s = Vec::new();
    for (i, n) in a.iter().enumerate() {
        // output n*x^i / -n*x^i
        if n.is_zero() { continue }

        let term = if i.is_zero() {
            n.to_str()
        } else if i == 1 {
            if (*n) == one { x.to_str() }
            else if (*n) == -one { format!("-{}", x) }
            else { format!("{}*{}", n.to_str(), x) }
        } else {
            if (*n) == one { format!("{}^{}", x, i) }
            else if (*n) == -one { format!("-{}^{}", x, i) }
            else { format!("{}*{}^{}", n.to_str(), x, i) }
        };

        if s.len() > 0 && (*n) > Zero::zero() { s.push(~"+"); }
        s.push(term);
    }

    s.concat()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_poly_add() {
        fn check(a: &[int], b: &[int], c: &[int]) {
            assert_eq!(super::add(a, b), c.to_owned());
            assert_eq!(super::add(b, a), c.to_owned());
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
            assert_eq!(super::mul(a, b), c.to_owned());
            assert_eq!(super::mul(b, a), c.to_owned());
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
        fn check(pol: &[int], f: |int| -> int) {
            for n in range(-10, 10) {
                assert_eq!(super::eval(pol, n), f(n));
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
        assert_eq!(super::to_str::<int>([], "x"), ~"0");
        assert_eq!(super::to_str([1], "x"), ~"1");
        assert_eq!(super::to_str([1, 1], "x"), ~"1+x");
        assert_eq!(super::to_str([1, 1, 1], "x"), ~"1+x+x^2");
        assert_eq!(super::to_str([2, 2, 2], "x"), ~"2+2*x+2*x^2");
        assert_eq!(super::to_str([0, 0, 0, 1], "x"), ~"x^3");
        assert_eq!(super::to_str([0, 0, 0, -1], "x"), ~"-x^3");
        assert_eq!(super::to_str([-1, 0, 0, -1], "x"), ~"-1-x^3");
        assert_eq!(super::to_str([-1, 1, 0, -1], "x"), ~"-1+x-x^3");
        assert_eq!(super::to_str([-1, 1, -1, -1], "x"), ~"-1+x-x^2-x^3");
    }
}
