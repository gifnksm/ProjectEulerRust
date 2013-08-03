use std::{vec, util};
use std::cmp::{Ord, Eq};
use std::ops::{Add, Mul};
use std::num::{Zero, One, Bounded};

pub trait Monoid {
    fn mempty() -> Self;
    fn mappend(&self, other: &Self) -> Self;
}

impl<T: Clone> Monoid for ~[T] {
    #[inline(always)]
    fn mempty() -> ~[T] { ~[] }
    #[inline(always)]
    fn mappend(&self, other: &~[T]) -> ~[T] { self + *other }
}

impl Monoid for ~str {
    #[inline(always)]
    fn mempty() -> ~str { ~"" }
    #[inline(always)]
    fn mappend(&self, other: &~str) -> ~str { *self + *other }
}


pub trait Wrap<T> {
    fn wrap(T) -> Self;
    fn unwrap(self) -> T;
    fn unwrap_ref<'a>(&'a self) -> &'a T;
}

pub trait WrapMonoid<T>: Wrap<T> + Monoid {}

#[deriving(Eq, Clone)]
pub struct Sum<T> { repr: T }

#[inline(always)]
pub fn Sum<T>(val: T) -> Sum<T> { Sum { repr: val } }

impl<T: Zero + Add<T, T>> Monoid for Sum<T> {
    #[inline(always)]
    fn mempty() -> Sum<T> { Sum(Zero::zero()) }
    #[inline(always)]
    fn mappend(&self, other: &Sum<T>) -> Sum<T> { Sum(self.repr + other.repr) }
}

impl<T> Wrap<T> for Sum<T> {
    #[inline(always)]
    fn wrap(val: T) -> Sum<T> { Sum(val) }
    #[inline(always)]
    fn unwrap(self) -> T { match self { Sum { repr: repr } => repr } }
    #[inline(always)]
    fn unwrap_ref<'a>(&'a self) -> &'a T { &self.repr}
}

impl<T: Zero + Add<T, T>> WrapMonoid<T> for Sum<T> {}


#[deriving(Eq, Clone)]
pub struct Prod<T> { repr: T }

#[inline(always)]
pub fn Prod<T>(val: T) -> Prod<T> { Prod { repr: val }}

impl<T: One + Mul<T, T>> Monoid for Prod<T> {
    #[inline(always)]
    fn mempty() -> Prod<T> { Prod(One::one()) }
    #[inline(always)]
    fn mappend(&self, other: &Prod<T>) -> Prod<T> { Prod(self.repr * other.repr) }
}

impl<T> Wrap<T> for Prod<T> {
    #[inline(always)]
    fn wrap(val: T) -> Prod<T> { Prod(val) }
    #[inline(always)]
    fn unwrap(self) -> T { match self { Prod { repr: repr } => repr } }
    #[inline(always)]
    fn unwrap_ref<'a>(&'a self) -> &'a T { &self.repr}
}

impl<T: One + Mul<T, T>> WrapMonoid<T> for Prod<T> {}


#[deriving(Eq, Clone)]
pub struct Max<T> { repr: T }

#[inline(always)]
pub fn Max<T>(val: T) -> Max<T> { Max{ repr: val } }

impl<T: Clone + Bounded + Ord> Monoid for Max<T> {
    #[inline(always)]
    fn mempty() -> Max<T> { Max(Bounded::min_value()) }
    #[inline(always)]
    fn mappend(&self, other: &Max<T>) -> Max<T> {
        if self.repr > other.repr { self.clone() } else { other.clone() }
    }
}

impl<T> Wrap<T> for Max<T> {
    #[inline(always)]
    fn wrap(val: T) -> Max<T> { Max(val) }
    #[inline(always)]
    fn unwrap(self) -> T { match self { Max { repr: repr } => repr } }
    #[inline(always)]
    fn unwrap_ref<'a>(&'a self) -> &'a T { &self.repr}
}

impl<T: Clone + Bounded + Ord> WrapMonoid<T> for Max<T> {}


#[deriving(Eq, Clone)]
pub struct Min<T> { repr: T }

#[inline(always)]
pub fn Min<T>(val: T) -> Min<T> { Min { repr: val } }

impl<T: Clone + Bounded + Ord> Monoid for Min<T> {
    #[inline(always)]
    fn mempty() -> Min<T> { Min(Bounded::max_value()) }
    #[inline(always)]
    fn mappend(&self, other: &Min<T>) -> Min<T> {
        if self.repr < other.repr { self.clone() } else { other.clone() }
    }
}

impl<T> Wrap<T> for Min<T> {
    #[inline(always)]
    fn wrap(val: T) -> Min<T> { Min(val) }
    #[inline(always)]
    fn unwrap(self) -> T { match self { Min { repr: repr } => repr } }
    #[inline(always)]
    fn unwrap_ref<'a>(&'a self) -> &'a T { &self.repr}
}

impl<T: Clone + Bounded + Ord> WrapMonoid<T> for Min<T> {}


#[inline(always)]
pub fn mconcat<T: Monoid>(v: &[T]) -> T {
    v.iter().fold(Monoid::mempty::<T>(), |accum, elem| { accum.mappend(elem) })
}

struct MergeMonoidIterator<V, T, U> {
    iter1: T,
    value1: Option<V>,
    iter2: U,
    value2: Option<V>
}

impl<V, T: Iterator<V>, U: Iterator<V>> MergeMonoidIterator<V, T, U> {
    #[inline(always)]
    pub fn new(iter1: T, iter2: U) -> MergeMonoidIterator<V, T, U> {
        MergeMonoidIterator { iter1: iter1, value1: None, iter2: iter2, value2: None }
    }
}

impl<K: TotalOrd, V: Monoid, T: Iterator<(K, V)>, U: Iterator<(K, V)>>
    Iterator<(K, V)> for MergeMonoidIterator<(K, V), T, U> {
    fn next(&mut self) -> Option<(K, V)> {
        fn return_val<V>(opt: &mut Option<V>) -> Option<V> {
            let mut result = None;
            util::swap(opt, &mut result);
            return result;
        }

        // Fill value if empty (if iter ends, set None)
        if self.value1.is_none() { self.value1 = self.iter1.next(); }
        if self.value2.is_none() { self.value2 = self.iter2.next(); }

        // If one value is None, returns anogher value (with setting None)
        if self.value1.is_none() { return return_val(&mut self.value2); }
        if self.value2.is_none() { return return_val(&mut self.value1); }

        // Returns smaller value
        let cmp = self.value1.get_ref().first_ref().cmp(self.value2.get_ref().first_ref());
        match cmp {
            Less    => { return return_val(&mut self.value1); }
            Greater => { return return_val(&mut self.value2); },
            Equal   => {
                let mut r1 = None;
                let mut r2 = None;
                util::swap(&mut self.value1, &mut r1);
                util::swap(&mut self.value2, &mut r2);
                let ((k, v1), (_, v2)) = (r1.unwrap(), r2.unwrap());
                return Some((k, v1.mappend(&v2)));
            }
        }
    }
}

struct MergeMultiMonoidIterator<V, T> {
    iters:  ~[T],
    values: ~[Option<V>]
}

impl<V, T: Iterator<V>> MergeMultiMonoidIterator<V, T> {
    #[inline(always)]
    pub fn new(iters: ~[T]) -> MergeMultiMonoidIterator<V, T> {
        MergeMultiMonoidIterator { values: vec::from_fn(iters.len(), |_| None), iters: iters }
    }
}

impl<K: TotalOrd, V: Monoid, T: Iterator<(K, V)>>
    Iterator<(K, V)> for MergeMultiMonoidIterator<(K, V), T> {
    fn next(&mut self) -> Option<(K, V)> {
        fn get_ref<'a, K, V>(opt: &'a Option<(K, V)>) -> &'a K { opt.get_ref().first_ref() }

        let len = self.iters.len();
        if len == 0 { return None; }

        // Fill value if empty (if iter ends, set None)
        foreach i in range(0, len) {
            if self.values[i].is_none() { self.values[i] = self.iters[i].next(); }
        }

        let mut min_idx = ~[];
        foreach i in range(0, len) {
            if self.values[i].is_none() { loop; }
            if min_idx.is_empty() { min_idx.push(i); loop; }

            match get_ref(&self.values[min_idx[0]]).cmp(get_ref(&self.values[i])) {
                Less    => {},
                Greater => { min_idx = ~[i];  }
                Equal   => { min_idx.push(i); }
            }
        }

        if min_idx.is_empty() { return None; }
        let mut result = None;
        foreach &i in min_idx.iter() {
            if result.is_none() {
                util::swap(&mut self.values[i], &mut result);
            } else {
                let mut r1 = None;
                let mut r2 = None;
                util::swap(&mut r1, &mut self.values[i]);
                util::swap(&mut r2, &mut result);
                let ((k, v1), (_, v2)) = (r1.unwrap(), r2.unwrap());
                result = Some((k, v1.mappend(&v2)));
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec;

    #[test]
    fn test_mconcat() {
        fn check_wrap<T: Eq + Clone, M: Monoid + Wrap<T>>(v: &[T], f: &fn(T) -> M, result: T) {
            let ms = v.to_owned().consume_iter().transform(f).collect::<~[M]>();
            assert_eq!(mconcat(ms).unwrap(), result);
        }

        let v1 = [1, 2, 3, 4];
        let v2 = [0, 1, 2, 3, 4];
        let v3 = [0, 0, 0, 0];
        let v4 = [];

        check_wrap(v1, Sum, 10);
        check_wrap(v2, Sum, 10);
        check_wrap(v3, Sum, 0);
        check_wrap(v4, Sum, 0);

        check_wrap(v1, Prod, 24);
        check_wrap(v2, Prod, 0);
        check_wrap(v3, Prod, 0);
        check_wrap(v4, Prod, 1);

        check_wrap(v1, Max, 4);
        check_wrap(v2, Max, 4);
        check_wrap(v3, Max, 0);

        check_wrap(v1, Min, 1);
        check_wrap(v2, Min, 0);
        check_wrap(v3, Min, 0);

        assert_eq!(mconcat([~[], ~[1, 2, 3], ~[4], ~[5]]), ~[1, 2, 3, 4, 5]);
        assert_eq!(mconcat([~"", ~"abc", ~"d", ~"e"]), ~"abcde");
    }

    fn to_sum <T: Clone, U: Clone>(tp: &(T, U)) -> (T, Sum<U>)  { (tp.n0(), Sum(tp.n1())) }
    fn to_prod<T: Clone, U: Clone>(tp: &(T, U)) -> (T, Prod<U>) { (tp.n0(), Prod(tp.n1())) }
    fn to_max <T: Clone, U: Clone>(tp: &(T, U)) -> (T, Max<U>)  { (tp.n0(), Max(tp.n1())) }
    fn to_min <T: Clone, U: Clone>(tp: &(T, U)) -> (T, Min<U>)  { (tp.n0(), Min(tp.n1())) }

    #[test]
    fn test_merge_monoid_iterator() {
        fn check<M: Monoid + Wrap<int> + Eq>(v1: &[(int, int)],
                                             v2: &[(int, int)],
                                             f: &fn(int) -> M,
                                             result: &[(int, int)]) {
            let merged: ~[(int, M)] = MergeMonoidIterator::new(
                v1.iter().transform(|&(x, y)| (x, f(y))),
                v2.iter().transform(|&(x, y)| (x, f(y)))
            ).collect();
            assert_eq!(merged, result.map(|&(x, y)| (x, f(y))));

            let merged: ~[(int, M)] = MergeMonoidIterator::new(
                v2.iter().transform(|&(x, y)| (x, f(y))),
                v1.iter().transform(|&(x, y)| (x, f(y)))
            ).collect();
            assert_eq!(merged, result.map(|&(x, y)| (x, f(y))));
        }

        let v1 = [(1, 1), (3, 1), (4, 3), (6, 1)];
        let v2 = [(1, 2), (2, 1), (4, 2), (7, 2)];

        check(v1, v2, Sum, [(1, 3), (2, 1), (3, 1), (4, 5), (6, 1), (7, 2)]);
        check(v1, v2, Prod, [(1, 2), (2, 1), (3, 1), (4, 6), (6, 1), (7, 2)]);
        check(v1, v2, Max, [(1, 2), (2, 1), (3, 1), (4, 3), (6, 1), (7, 2)]);
        check(v1, v2, Min, [(1, 1), (2, 1), (3, 1), (4, 2), (6, 1), (7, 2)]);

        check(v1, [], Sum, v1);
        check([], [], Sum, []);
        check(v1, [], Prod, v1);
        check([], [], Prod, []);
        check(v1, [], Max, v1);
        check([], [], Max, []);
        check(v1, [], Min, v1);
        check([], [], Min, []);
    }

    #[test]
    fn test_merge_multi_monoid_iterator() {
        fn check<M: Monoid + Wrap<int> + Eq + Clone>(vs: &[~[(int, int)]],
                                                     f: &fn(int) -> M,
                                                     result: &[(int, int)]) {
            for vec::each_permutation(vs) |vs| {
                let vs = vs.map(|ks| ks.map(|&(x, y)| (x, f(y))).consume_iter());
                let merged = MergeMultiMonoidIterator::new(vs).collect::<~[(int, M)]>();
                assert_eq!(merged, result.map(|&(x, y)| (x, f(y))));
            }
        }

        {
            let v = [~[(1, 1), (2, 1), (3, 5), (4, 0), (5, 3)],
                     ~[(1, 2), (3, 2)],
                     ~[(-1, 3), (4, 2), (7, 1)],
                     ~[]];
            check(v, Sum, [(-1, 3), (1, 3), (2, 1), (3, 7), (4, 2), (5, 3), (7, 1)]);
            check(v, Prod, [(-1, 3), (1, 2), (2, 1), (3, 10), (4, 0), (5, 3), (7, 1)]);
            check(v, Max, [(-1, 3), (1, 2), (2, 1), (3, 5), (4, 2), (5, 3), (7, 1)]);
            check(v, Min, [(-1, 3), (1, 1), (2, 1), (3, 2), (4, 0), (5, 3), (7, 1)]);
        }

        {
            let v = [~[(1, 2), (2, 1)],
                     ~[(1, 3)],
                     ~[(1, 0)],
                     ~[(1, -4)]];
            check(v, Sum,  [(1, 1), (2, 1)]);
            check(v, Prod, [(1, 0), (2, 1)]);
            check(v, Max,  [(1, 3), (2, 1)]);
            check(v, Min,  [(1, -4), (2, 1)]);
        }

        {
            let v = [~[], ~[], ~[]];
            check(v, Sum, []);
            check(v, Prod, []);
            check(v, Max, []);
            check(v, Min, []);
        }
    }
}
