use core::util;
use core::cmp::{ Ord, Eq };
use core::ops::{ Add, Mul };
use core::num::{ Zero, One };
use core::iterator::{ Iterator, IteratorUtil, MapIterator };

use common::bounded::{ Bounded };

pub trait Monoid {
    fn mempty() -> Self;
    fn mappend(&self, other: &Self) -> Self;
}

pub trait Unwrap<T> {
    fn unwrap(&self) -> T;
}

pub struct Sum<T> { repr: T }
pub fn Sum<T>(val: T) -> Sum<T> { Sum { repr: val } }

impl<T: Zero+Add<T,T>+Copy> Monoid for Sum<T> {
    fn mempty() -> Sum<T> { Sum(Zero::zero()) }
    fn mappend(&self, other: &Sum<T>) -> Sum<T> { Sum(self.repr + other.repr) }
}

impl<T: Copy> Unwrap<T> for Sum<T> {
    fn unwrap(&self) -> T { self.repr }
}

impl<T: Eq> Eq for Sum<T> {
    fn eq(&self, other: &Sum<T>) -> bool { self.repr == other.repr }
    fn ne(&self, other: &Sum<T>) -> bool { self.repr != other.repr }
}

pub struct Prod<T> { repr: T }
pub fn Prod<T>(val: T) -> Prod<T> { Prod { repr: val }}

impl<T: One+Mul<T,T>+Copy> Monoid for Prod<T> {
    fn mempty() -> Prod<T> { Prod(One::one()) }
    fn mappend(&self, other: &Prod<T>) -> Prod<T> { Prod(self.repr * other.repr) }
}

impl<T: Copy> Unwrap<T> for Prod<T> {
    fn unwrap(&self) -> T { self.repr }
}

impl<T: cmp::Eq> Eq for Prod<T> {
    fn eq(&self, other: &Prod<T>) -> bool { self.repr == other.repr }
    fn ne(&self, other: &Prod<T>) -> bool { self.repr != other.repr }
}

pub struct Max<T> { repr: T }
pub fn Max<T>(val: T) -> Max<T> { Max{ repr: val } }

impl<T: Copy+Bounded+Ord> Monoid for Max<T> {
    fn mempty() -> Max<T> { Max(Bounded::min_value()) }
    fn mappend(&self, other: &Max<T>) -> Max<T> {
        if self.repr < other.repr { *other } else { *self }
    }
}

impl<T: Copy> Unwrap<T> for Max<T> {
    fn unwrap(&self) -> T { self.repr }
}

impl<T: Eq> Eq for Max<T> {
    fn eq(&self, other: &Max<T>) -> bool { self.repr == other.repr }
    fn ne(&self, other: &Max<T>) -> bool { self.repr != other.repr }
}

pub struct Min<T> { repr: T }
pub fn Min<T>(val: T) -> Min<T> { Min { repr: val } }

impl<T: Copy+Bounded+Ord> Monoid for Min<T> {
    fn mempty() -> Min<T> { Min(Bounded::max_value()) }
    fn mappend(&self, other: &Min<T>) -> Min<T> {
        if self.repr > other.repr { *other } else { *self }
    }
}

impl<T: Copy> Unwrap<T> for Min<T> {
    fn unwrap(&self) -> T { self.repr }
}


impl<T: Eq> Eq for Min<T> {
    fn eq(&self, other: &Min<T>) -> bool { self.repr == other.repr }
    fn ne(&self, other: &Min<T>) -> bool { self.repr != other.repr }
}

pub fn mconcat<T: Copy+Monoid>(v: &[T]) -> T {
    vec::foldl(Monoid::mempty(), v, |accum, elt| { elt.mappend(&accum) })
}

struct MergeMonoidIterator<V, T, U> {
    iter1: T,
    value1: Option<V>,
    iter2: U,
    value2: Option<V>
}

impl<V, T: Iterator<V>, U: Iterator<V>> MergeMonoidIterator<V, T, U> {
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
                let mut r1 = None, r2 = None;
                util::swap(&mut self.value1, &mut r1);
                util::swap(&mut self.value2, &mut r2);
                let ((k, v1), (_, v2)) = (r1.unwrap(), r2.unwrap());
                return Some((k, v1.mappend(&v2)));
            }
        }
    }
}

pub fn merge_monoid_as<K: TotalOrd, V, MT, M: Monoid + Unwrap<MT>, T: Iterator<(K, V)>, U: Iterator<(K, V)>, X>
    (it1: T, it2: U, conv: &fn(V) -> M,
     f: &fn(MapIterator<(K, M), (K, MT),
            MergeMonoidIterator<(K, M), MapIterator<(K, V), (K, M), T>,
            MapIterator<(K, V), (K, M), U>>>) -> X
    ) -> X {
    let it1 = it1.transform(|(k, v)| (k, conv(v)));
    let it2 = it2.transform(|(k, v)| (k, conv(v)));
    let it = MergeMonoidIterator::new(it1, it2);
    return f(it.transform(|(k, m)| (k, m.unwrap())));
}

pub fn merge<T: Copy + Ord, M: Copy + Monoid>(
    vec1: &[(T, M)], vec2: &[(T, M)]
) -> ~[(T, M)] {
    let (l1, l2) = (vec1.len(), vec2.len());

    let mut result = ~[];
    let mut (i1, i2) = (0, 0);
    while i1 < l1 && i2 < l2 {
        let (v1, m1) = vec1[i1];
        let (v2, m2) = vec2[i2];
        if v1 < v2 {
            result.push((v1, m1));
            i1 += 1;
            loop;
        }
        if v2 < v1 {
            result.push((v2, m2));
            i2 += 1;
            loop;
        }
        result.push((v1, m1.mappend(&m2)));
        i1 += 1;
        i2 += 1;
    }

    if i1 < l1 { result += vec1.slice(i1, l1); }
    if i2 < l2 { result += vec2.slice(i2, l2); }
    return result;
}

pub fn mergei<T: Copy+Ord, M: Copy+Monoid>(vecs: &[~[(T, M)]]) -> ~[(T, M)] {
    return match vecs.len() {
      0u => ~[],
      1u => ~[] + vecs[0],
      len  => {
        let pre  = mergei(vecs.slice(0u, len / 2u));
        let post = mergei(vecs.slice(len / 2u, len));
        merge(pre, post)
      }
    }
}

pub fn merge_as<T: Copy+Ord, U: Copy, MT, M: Copy+Monoid+Unwrap<MT>>
    (vec1: &[(T, U)], vec2: &[(T, U)], f: &fn(U) -> M) -> ~[(T, MT)] {
    fn convert<T: Copy, U: Copy, M: Copy>(v: &[(T, U)], f: &fn(U) -> M) -> ~[(T, M)] {
        do v.map |tp| { (tp.first(), f(tp.second())) }
    }
    return merge(convert(vec1, f), convert(vec2, f)).map(|tp| (tp.first(), tp.second().unwrap()));
}

pub fn mergei_as<T: Copy+Ord, U: Copy, MT, M: Copy+Monoid+Unwrap<MT>>(vecs: &[~[(T, U)]], f: &fn(U) -> M) -> ~[(T, MT)] {
    return mergei(
        vecs.map(|v| v.map(|tp| (tp.first(), f(tp.second()))))
    ).map(|tp| (tp.first(), tp.second().unwrap()));
}


#[cfg(test)]
mod tests {
    use super::*;
    use common::extvec;

    fn to_sum<T, U>(&tp: &(T, U)) -> (T, Sum<U>) {
        let (t, u) = tp;
        return (t, Sum(u))
    }

    fn to_prod<T, U>(&tp: &(T, U)) -> (T, Prod<U>) {
        let (t, u) = tp;
        return (t, Prod(u));
    }

    fn to_max<T, U>(&tp: &(T, U)) -> (T, Max<U>) {
        let (t, u) = tp;
        return (t, Max(u));
    }

    fn to_min<T, U>(&tp: &(T, U)) -> (T, Min<U>) {
        let (t, u) = tp;
        return (t, Min(u));
    }

    #[test]
    fn test_merge_monoid_iterator() {
        fn check<M: Monoid + Unwrap<int> + Eq>(v1: &[(int, int)],
                                               v2: &[(int, int)],
                                               f: &fn(int) -> M,
                                               result: &[(int, int)]) {
            let conv = |&(x, y): &(int, int)| (x, f(y));
            assert_eq!(
                extvec::from_iter(MergeMonoidIterator::new(v1.iter().transform(conv),
                                                           v2.iter().transform(conv))),
                result.map(conv));
            assert_eq!(
                extvec::from_iter(MergeMonoidIterator::new(v2.iter().transform(conv),
                                                           v1.iter().transform(conv))),
                result.map(conv));

            assert_eq!(merge_monoid_as(v1.iter().transform(|x| *x),
                                       v2.iter().transform(|x| *x),
                                       f, |x| extvec::from_iter(x)),
                       result.to_vec());

            assert_eq!(merge_monoid_as(v2.iter().transform(|x| *x),
                                       v1.iter().transform(|x| *x),
                                       f, |x| extvec::from_iter(x)),
                       result.to_vec());
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
    fn test_merge() {
        let arg1 = [(1, 1), (3, 1), (4, 1), (6, 1)];
        let arg2 = [(1, 2), (2, 1), (4, 1), (7, 2)];

        {
            let result = ~[(1, 3), (2, 1), (3, 1), (4, 2), (6, 1), (7, 2)];
            assert_eq!(merge(arg1.map(to_sum), arg2.map(to_sum)),
                       result.map(to_sum));
            assert_eq!(merge_as(arg1, arg2, Sum), result);
        }

        {
            let result = ~[(1, 2), (2, 1), (3, 1), (4, 1), (6, 1), (7, 2)];
            assert_eq!(merge(arg1.map(to_max), arg2.map(to_max)),
                       result.map(to_max));
            assert_eq!(merge_as(arg1, arg2, Max),
                       result);
        }

        {
            let result = arg1.map(to_sum);
            assert_eq!(merge(result, []), result);
        }

        {
            let result: ~[(int, Sum<int>)] = ~[];
            assert_eq!(merge([], []), result);
        }
    }

    #[test]
    fn test_mergei() {
        {
            let arg = [~[(1, 1), (2, 1)], ~[(1, 2), (3, 1)], ~[(-1, 3)]];
            let result = ~[(-1, 3), (1, 3), (2, 1), (3, 1)];
            assert_eq!(mergei(arg.map(|v| v.map(to_sum))), result.map(to_sum));
            assert_eq!(mergei_as(arg, Sum), result);
        }

        {
            let arg = [~[(1, 1)], ~[(1, 2)], ~[(1, 3)]];
            let result = ~[(1, 6)];
            assert_eq!(mergei(arg.map(|v| v.map(to_sum))), result.map(to_sum));
            assert_eq!(mergei_as(arg, Sum), result);
        }

        {
            let arg = [~[], ~[], ~[]];
            let result: ~[(int, int)] = ~[];
            assert_eq!(mergei(arg.map(|v| v.map(to_sum))),
                       result.map(to_sum));
            assert_eq!(mergei_as(arg, Sum),
                       result);
        }
    }
}

