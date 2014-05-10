use std::{cmp, fmt, num};
use std::num::{Zero, One};

#[deriving(Eq, Clone, Show)]
pub struct Poly<T> { data: Vec<T> }

impl<T: Zero> Poly<T> {
    #[inline]
    pub fn new(mut data: Vec<T>) -> Poly<T> {
        while data.last().map(|x| x.is_zero()).unwrap_or(false) {
            data.pop();
        }
        Poly { data: data }
    }
}

impl<T: Clone + Zero> Poly<T> {
    #[inline]
    pub fn from_slice(data: &[T]) -> Poly<T> {
        Poly { data: Vec::from_slice(omit_zeros(data))}
    }
}

impl<T: Zero + One> Poly<T> {
    #[inline]
    pub fn eval(&self, x: T) -> T {
        let mut sum = num::zero::<T>();
        let mut x_n = num::one::<T>();
        for n in self.data.iter() {
            sum = sum + (*n) * x_n;
            x_n = x_n * x;
        }
        sum
    }
}

impl<T> Poly<T> {
    #[inline]
    pub fn as_slice<'a>(&'a self) -> &'a [T] { self.data.as_slice() }
    #[inline]
    pub fn into_vec(self) -> Vec<T> { self.data }
}

impl<T: Zero + One + Eq + Neg<T> + Ord + fmt::Show> Poly<T> {
    pub fn pretty(&self, x: &str) -> ~str {
        if self.is_zero() { return "0".to_owned() }

        let one = One::one();
        let mut s = Vec::new();
        for (i, n) in self.data.iter().enumerate() {
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

            if s.len() > 0 && (*n) > Zero::zero() { s.push("+".to_owned()); }
            s.push(term);
        }

        s.concat()
    }
}

impl<T: Zero> Zero for Poly<T> {
    #[inline]
    fn zero() -> Poly<T> { Poly { data: vec![] } }
    #[inline]
    fn is_zero(&self) -> bool { self.data.is_empty() }
}

impl<T: Zero + One> One for Poly<T> {
    #[inline]
    fn one() -> Poly<T> { Poly { data: vec![One::one()] } }
}

impl<T: Neg<T> + Zero> Neg<Poly<T>> for Poly<T> {
    #[inline]
    fn neg(&self) -> Poly<T> {
        Poly::new(self.data.iter().map(|x| -(*x)).collect())
    }
}

impl<T: Zero + Add<T, T>> Add<Poly<T>, Poly<T>> for Poly<T> {
    fn add(&self, other: &Poly<T>) -> Poly<T> {
        let bigger = if self.data.len() <= other.data.len() { other } else { self };
        let min_len = cmp::min(self.data.len(), other.data.len());
        let max_len = cmp::max(self.data.len(), other.data.len());

        let mut sum = Vec::with_capacity(max_len);
        for i in range(0, min_len) {
            sum.push(*self.data.get(i) + *other.data.get(i));
        }
        for i in range(min_len, max_len) {
            sum.push(*bigger.data.get(i) + Zero::zero())
        }
        Poly::new(sum)
    }
}

impl<T: Zero + Sub<T, T>> Sub<Poly<T>, Poly<T>> for Poly<T> {
    fn sub(&self, other: &Poly<T>) -> Poly<T> {
        let min_len = cmp::min(self.data.len(), other.data.len());
        let max_len = cmp::max(self.data.len(), other.data.len());

        let mut sub = Vec::with_capacity(max_len);
        for i in range(0, min_len) {
            sub.push(*self.data.get(i) - *other.data.get(i));
        }
        if self.data.len() <= other.data.len() {
            for i in range(min_len, max_len) {
                sub.push(num::zero::<T>() - *other.data.get(i))
            }
        } else {
            for i in range(min_len, max_len) {
                sub.push(*self.data.get(i) + Zero::zero())
            }
        }
        Poly::new(sub)
    }
}

impl<T: Zero + Mul<T, T>> Mul<Poly<T>, Poly<T>> for Poly<T> {
    fn mul(&self, other: &Poly<T>) -> Poly<T> {
        if self.is_zero() || other.is_zero() { return Zero::zero() }

        let mut prod: Vec<T> = Vec::from_fn(self.data.len() + other.data.len() - 1, |_| Zero::zero());
        for (i, n) in self.data.iter().enumerate() {
            for (j, m) in other.data.iter().enumerate() {
                *prod.get_mut(i + j) = *prod.get(i + j) + (*n) * (*m);
            }
        }
        Poly::new(prod)
    }
}

fn omit_zeros<'a, T: Zero>(v: &'a [T]) -> &'a [T] {
    let len = v.iter().rposition(|n| !n.is_zero()).map_or(0, |p| p + 1);
    return v.slice(0, len);
}

pub fn add<T: Zero + Add<T,T> + Clone>(a: &[T], b: &[T]) -> Vec<T> {
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
    sum
}

pub fn mul<T: Zero + Add<T, T> + Mul<T, T>>(a: &[T], b: &[T]) -> Vec<T> {
    let a = omit_zeros(a);
    let b = omit_zeros(b);

    if a.is_empty() || b.is_empty() { return vec![]; }
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
    if a.is_empty() { return "0".to_owned(); }

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

        if s.len() > 0 && (*n) > Zero::zero() { s.push("+".to_owned()); }
        s.push(term);
    }

    s.concat()
}

#[cfg(test)]
mod tests {
    mod poly {
        use super::super::Poly;

        #[test]
        fn new() {
            assert_eq!(vec![1, 2, 3], Poly::new(vec![1, 2, 3]).data);
            assert_eq!(vec![1, 2, 3], Poly::new(vec![1, 2, 3, 0, 0]).data);
            assert_eq!(vec![], Poly::new(vec![0, 0, 0]).data);
        }

        #[test]
        fn neg_add_sub() {
            fn check(a: &[int], b: &[int], c: &[int]) {
                fn check_eq(a: &Poly<int>, b: &Poly<int>) {
                    assert_eq!(*a, *b);
                    assert_eq!(-(*a), -(*b));
                }
                fn check_add(sum: &Poly<int>, a: &Poly<int>, b: &Poly<int>) {
                    check_eq(sum, &((*a) + (*b)));
                    check_eq(sum, &((*b) + (*a)));
                }
                fn check_sub(sum: &Poly<int>, a: &Poly<int>, b: &Poly<int>) {
                    check_eq(a, &((*sum) - (*b)));
                    check_eq(b, &((*sum) - (*a)));
                }

                let a = Poly::from_slice(a);
                let b = Poly::from_slice(b);
                let c = Poly::from_slice(c);
                check_add(&c, &a, &b);
                check_add(&(-c), &(-a), &(-b));
                check_sub(&c, &a, &b);
                check_sub(&(-c), &(-a), &(-b));
            }
            check([], [], []);
            check([], [1], [1]);
            check([1], [1], [2]);
            check([1, 0, 1], [1], [2, 0, 1]);
            check([1, 0, -1], [-1, 0, 1], []);
        }

        #[test]
        fn mul() {
            fn check(a: &[int], b: &[int], c: &[int]) {
                let a = Poly::from_slice(a);
                let b = Poly::from_slice(b);
                let c = Poly::from_slice(c);
                assert_eq!(c, a * b);
                assert_eq!(c, b * a);
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
        fn eval() {
            fn check(pol: &[int], f: |int| -> int) {
                for n in range(-10, 10) {
                    assert_eq!(f(n), Poly::from_slice(pol).eval(n));
                }
            }
            check([], |_x| 0);
            check([1], |_x| 1);
            check([1, 1], |x| x + 1);
            check([0, 1], |x| x);
            check([10, -10, 10], |x| 10*x*x - 10 * x + 10);
        }

        #[test]
        fn pretty() {
            assert_eq!(Poly::from_slice([0]).pretty("x"), "0".to_owned());
            assert_eq!(Poly::from_slice([1]).pretty("x"), "1".to_owned());
            assert_eq!(Poly::from_slice([1, 1]).pretty("x"), "1+x".to_owned());
            assert_eq!(Poly::from_slice([1, 1, 1]).pretty("x"), "1+x+x^2".to_owned());
            assert_eq!(Poly::from_slice([2, 2, 2]).pretty("x"), "2+2*x+2*x^2".to_owned());
            assert_eq!(Poly::from_slice([0, 0, 0, 1]).pretty("x"), "x^3".to_owned());
            assert_eq!(Poly::from_slice([0, 0, 0, -1]).pretty("x"), "-x^3".to_owned());
            assert_eq!(Poly::from_slice([-1, 0, 0, -1]).pretty("x"), "-1-x^3".to_owned());
            assert_eq!(Poly::from_slice([-1, 1, 0, -1]).pretty("x"), "-1+x-x^3".to_owned());
            assert_eq!(Poly::from_slice([-1, 1, -1, -1]).pretty("x"), "-1+x-x^2-x^3".to_owned());
        }
    }
    #[test]
    fn test_poly_add() {
        fn check(a: &[int], b: &[int], c: &[int]) {
            assert_eq!(super::add(a, b).as_slice(), c);
            assert_eq!(super::add(b, a).as_slice(), c);
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
            assert_eq!(super::mul(a, b).as_slice(), c);
            assert_eq!(super::mul(b, a).as_slice(), c);
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
    fn test_poly_eval() {
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
        assert_eq!(super::to_str::<int>([], "x"), "0".to_owned());
        assert_eq!(super::to_str([1], "x"), "1".to_owned());
        assert_eq!(super::to_str([1, 1], "x"), "1+x".to_owned());
        assert_eq!(super::to_str([1, 1, 1], "x"), "1+x+x^2".to_owned());
        assert_eq!(super::to_str([2, 2, 2], "x"), "2+2*x+2*x^2".to_owned());
        assert_eq!(super::to_str([0, 0, 0, 1], "x"), "x^3".to_owned());
        assert_eq!(super::to_str([0, 0, 0, -1], "x"), "-x^3".to_owned());
        assert_eq!(super::to_str([-1, 0, 0, -1], "x"), "-1-x^3".to_owned());
        assert_eq!(super::to_str([-1, 1, 0, -1], "x"), "-1+x-x^3".to_owned());
        assert_eq!(super::to_str([-1, 1, -1, -1], "x"), "-1+x-x^2-x^3".to_owned());
    }
}
