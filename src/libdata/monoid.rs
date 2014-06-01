use std::mem;
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

pub struct MergeMonoidIterator<V, T, U> {
    iter1: T,
    value1: Option<V>,
    iter2: U,
    value2: Option<V>
}

impl<V, T: Iterator<V>, U: Iterator<V>> MergeMonoidIterator<V, T, U> {
    #[inline]
    pub fn new(iter1: T, iter2: U) -> MergeMonoidIterator<V, T, U> {
        MergeMonoidIterator { iter1: iter1, value1: None, iter2: iter2, value2: None }
    }
}

impl<K: TotalOrd, V: Monoid, T: Iterator<(K, V)>, U: Iterator<(K, V)>>
    Iterator<(K, V)> for MergeMonoidIterator<(K, V), T, U> {
    fn next(&mut self) -> Option<(K, V)> {
        fn return_val<V>(opt: &mut Option<V>) -> Option<V> {
            let mut result = None;
            mem::swap(opt, &mut result);
            return result;
        }

        // Fill value if empty (if iter ends, set None)
        if self.value1.is_none() { self.value1 = self.iter1.next(); }
        if self.value2.is_none() { self.value2 = self.iter2.next(); }

        // If one value is None, returns anogher value (with setting None)
        if self.value1.is_none() { return return_val(&mut self.value2); }
        if self.value2.is_none() { return return_val(&mut self.value1); }

        // Returns smaller value
        let cmp = self.value1.get_ref().ref0().cmp(self.value2.get_ref().ref0());
        match cmp {
            Less    => { return return_val(&mut self.value1); }
            Greater => { return return_val(&mut self.value2); },
            Equal   => {
                let mut r1 = None;
                let mut r2 = None;
                mem::swap(&mut self.value1, &mut r1);
                mem::swap(&mut self.value2, &mut r2);
                let ((k, v1), (_, v2)) = (r1.unwrap(), r2.unwrap());
                return Some((k, v1.mappend(&v2)));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Monoid, Wrap, Sum, Prod, Max, Min,
                MergeMonoidIterator};
    use std::fmt::Show;

    #[test]
    fn test_mconcat() {
        fn check_wrap<T: TotalEq + Clone + Show, M: Monoid + Wrap<T>>(v: &[T], f: |T| -> M, result: T) {
            let ms = v.to_owned().move_iter().map(f).collect::<Vec<M>>();
            assert_eq!(super::mconcat(ms.as_slice()).unwrap(), result);
        }

        let v1 = [1, 2, 3, 4];
        let v2 = [0, 1, 2, 3, 4];
        let v3 = [0, 0, 0, 0];
        let v4 = [];

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

        assert_eq!(super::mconcat([vec![], vec![1, 2, 3], vec![4], vec![5]]), vec![1, 2, 3, 4, 5]);
        assert_eq!(super::mconcat(["".to_string(), "abc".to_string(), "d".to_string(), "e".to_string()]),
                   "abcde".to_string());
    }

    #[test]
    fn test_merge_monoid_iterator() {
        fn check<M: Monoid + Wrap<int> + TotalEq + Show>(v1: &[(int, int)],
                                                    v2: &[(int, int)],
                                                    f: fn(int) -> M,
                                                    result: &[(int, int)]) {
            let merged = MergeMonoidIterator::new(
                v1.iter().map(|&(x, y)| (x, f(y))),
                v2.iter().map(|&(x, y)| (x, f(y)))
            ).collect::<Vec<(int, M)>>();
            assert_eq!(merged, result.iter().map(|&(x, y)| (x, f(y))).collect());

            let merged = MergeMonoidIterator::new(
                v2.iter().map(|&(x, y)| (x, f(y))),
                v1.iter().map(|&(x, y)| (x, f(y)))
            ).collect::<Vec<(int, M)>>();
            assert_eq!(merged, result.iter().map(|&(x, y)| (x, f(y))).collect());
        }

        let v1 = [(1, 1), (3, 1), (4, 3), (6, 1)];
        let v2 = [(1, 2), (2, 1), (4, 2), (7, 2)];

        check(v1, v2, Sum::new, [(1, 3), (2, 1), (3, 1), (4, 5), (6, 1), (7, 2)]);
        check(v1, v2, Prod::new, [(1, 2), (2, 1), (3, 1), (4, 6), (6, 1), (7, 2)]);
        check(v1, v2, Max::new, [(1, 2), (2, 1), (3, 1), (4, 3), (6, 1), (7, 2)]);
        check(v1, v2, Min::new, [(1, 1), (2, 1), (3, 1), (4, 2), (6, 1), (7, 2)]);

        check(v1, [], Sum::new, v1);
        check([], [], Sum::new, []);
        check(v1, [], Prod::new, v1);
        check([], [], Prod::new, []);
        check(v1, [], Max::new, v1);
        check([], [], Max::new, []);
        check(v1, [], Min::new, v1);
        check([], [], Min::new, []);
    }
}
