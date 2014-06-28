use std::cmp::{Ord, Eq};
use std::ops::{Add, Mul};
use std::num::{Zero, One, Bounded};

pub trait Monoid {
    fn mempty() -> Self;
    fn mappend(&self, other: &Self) -> Self;
}

impl<T: Clone> Monoid for Vec<T> {
    #[inline]
    fn mempty() -> Vec<T> { Vec::new() }
    #[inline]
    fn mappend(&self, other: &Vec<T>) -> Vec<T> { self + *other }
}

impl Monoid for String {
    #[inline]
    fn mempty() -> String { "".to_string() }
    #[inline]
    fn mappend(&self, other: &String) -> String { format!("{}{}", self, other) }
}


pub trait Wrap<T> {
    fn wrap(T) -> Self;
    fn unwrap(self) -> T;
    fn unwrap_ref<'a>(&'a self) -> &'a T;
}

pub trait WrapMonoid<T>: Wrap<T> + Monoid {}

#[deriving(Eq, PartialEq, Clone, Show)]
pub struct Sum<T> { repr: T }

impl<T> Sum<T> {
    #[inline]
    pub fn new(val: T) -> Sum<T> { Sum { repr: val }}
}

impl<T: Zero + Add<T, T>> Monoid for Sum<T> {
    #[inline]
    fn mempty() -> Sum<T> { Sum::new(Zero::zero()) }
    #[inline]
    fn mappend(&self, other: &Sum<T>) -> Sum<T> { Sum::new(self.repr + other.repr) }
}

impl<T> Wrap<T> for Sum<T> {
    #[inline]
    fn wrap(val: T) -> Sum<T> { Sum::new(val) }
    #[inline]
    fn unwrap(self) -> T { match self { Sum { repr: repr } => repr } }
    #[inline]
    fn unwrap_ref<'a>(&'a self) -> &'a T { &self.repr}
}

impl<T: Zero + Add<T, T>> WrapMonoid<T> for Sum<T> {}


#[deriving(Eq, PartialEq, Clone, Show)]
pub struct Prod<T> { repr: T }

impl<T> Prod<T> {
    #[inline]
    pub fn new(val: T) -> Prod<T> { Prod { repr: val }}
}

impl<T: One + Mul<T, T>> Monoid for Prod<T> {
    #[inline]
    fn mempty() -> Prod<T> { Prod::new(One::one()) }
    #[inline]
    fn mappend(&self, other: &Prod<T>) -> Prod<T> { Prod::new(self.repr * other.repr) }
}

impl<T> Wrap<T> for Prod<T> {
    #[inline]
    fn wrap(val: T) -> Prod<T> { Prod::new(val) }
    #[inline]
    fn unwrap(self) -> T { match self { Prod { repr: repr } => repr } }
    #[inline]
    fn unwrap_ref<'a>(&'a self) -> &'a T { &self.repr}
}

impl<T: One + Mul<T, T>> WrapMonoid<T> for Prod<T> {}


#[deriving(Eq, PartialEq, Clone, Show)]
pub struct Max<T> { repr: T }

impl<T> Max<T> {
    #[inline]
    pub fn new(val: T) -> Max<T> { Max { repr: val } }
}

impl<T: Clone + Bounded + Ord> Monoid for Max<T> {
    #[inline]
    fn mempty() -> Max<T> { Max::new(Bounded::min_value()) }
    #[inline]
    fn mappend(&self, other: &Max<T>) -> Max<T> {
        if self.repr > other.repr { self.clone() } else { other.clone() }
    }
}

impl<T> Wrap<T> for Max<T> {
    #[inline]
    fn wrap(val: T) -> Max<T> { Max::new(val) }
    #[inline]
    fn unwrap(self) -> T { match self { Max { repr: repr } => repr } }
    #[inline]
    fn unwrap_ref<'a>(&'a self) -> &'a T { &self.repr}
}

impl<T: Clone + Bounded + Ord> WrapMonoid<T> for Max<T> {}


#[deriving(Eq, PartialEq, Clone, Show)]
pub struct Min<T> { repr: T }

impl<T> Min<T> {
    #[inline]
    pub fn new(val: T) -> Min<T> { Min { repr: val } }
}

impl<T: Clone + Bounded + Ord> Monoid for Min<T> {
    #[inline]
    fn mempty() -> Min<T> { Min::new(Bounded::max_value()) }
    #[inline]
    fn mappend(&self, other: &Min<T>) -> Min<T> {
        if self.repr < other.repr { self.clone() } else { other.clone() }
    }
}

impl<T> Wrap<T> for Min<T> {
    #[inline]
    fn wrap(val: T) -> Min<T> { Min::new(val) }
    #[inline]
    fn unwrap(self) -> T { match self { Min { repr: repr } => repr } }
    #[inline]
    fn unwrap_ref<'a>(&'a self) -> &'a T { &self.repr}
}

impl<T: Clone + Bounded + Ord> WrapMonoid<T> for Min<T> {}


#[inline]
pub fn mconcat<T: Monoid>(v: &[T]) -> T {
    v.iter().fold(Monoid::mempty(), |accum: T, elem| { accum.mappend(elem) })
}

#[cfg(test)]
mod tests {
    use super::{Monoid, Wrap, Sum, Prod, Max, Min};
    use std::fmt::Show;

    #[test]
    fn test_mconcat() {
        fn check_wrap<T: Eq + Clone + Show, M: Monoid + Wrap<T>>(v: &[T], f: |T| -> M, result: T) {
            let ms = v.iter().map(|x| x.clone()).map(f).collect::<Vec<M>>();
            assert_eq!(super::mconcat(ms.as_slice()).unwrap(), result);
        }

        let v1 = [1u, 2, 3, 4];
        let v2 = [0u, 1, 2, 3, 4];
        let v3 = [0u, 0, 0, 0];
        let v4: &[uint] = [];

        check_wrap(v1, Sum::new, 10);
        check_wrap(v2, Sum::new, 10);
        check_wrap(v3, Sum::new, 0);
        check_wrap(v4, Sum::new, 0);

        check_wrap(v1, Prod::new, 24);
        check_wrap(v2, Prod::new, 0);
        check_wrap(v3, Prod::new, 0);
        check_wrap(v4, Prod::new, 1);

        check_wrap(v1, Max::new, 4);
        check_wrap(v2, Max::new, 4);
        check_wrap(v3, Max::new, 0);

        check_wrap(v1, Min::new, 1);
        check_wrap(v2, Min::new, 0);
        check_wrap(v3, Min::new, 0);

        assert_eq!(super::mconcat([vec![], vec![1u, 2, 3], vec![4], vec![5]]), vec![1, 2, 3, 4, 5]);
        assert_eq!(super::mconcat(["".to_string(), "abc".to_string(), "d".to_string(), "e".to_string()]),
                   "abcde".to_string());
    }
}
