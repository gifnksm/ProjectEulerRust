use bounded::{ Bounded };

use core::cmp::{ Ord, Eq };
use core::ops::{ Add, Mul };
use core::num::{ Zero, One };

pub trait Monoid {
    static pure fn mempty() -> Self;
    pure fn mappend(&self, other: &Self) -> Self;
}

pub trait Unwrap<T> {
    pure fn unwrap(&self) -> T;
}

pub struct Sum<T> { repr: T }
pub pure fn Sum<T>(val: T) -> Sum<T> { Sum { repr: val } }

impl<T: Zero+Add<T,T>+Copy> Monoid for Sum<T> {
    static pure fn mempty() -> Sum<T> { Sum(Zero::zero()) }
    pure fn mappend(&self, other: &Sum<T>) -> Sum<T> { Sum(self.repr + other.repr) }
}

impl<T: Copy> Unwrap<T> for Sum<T> {
    pure fn unwrap(&self) -> T { self.repr }
}

impl<T: Eq> Eq for Sum<T> {
    pure fn eq(&self, other: &Sum<T>) -> bool { self.repr == other.repr }
    pure fn ne(&self, other: &Sum<T>) -> bool { self.repr != other.repr }
}

pub struct Prod<T> { repr: T }
pub pure fn Prod<T>(val: T) -> Prod<T> { Prod { repr: val }}

impl<T: One+Mul<T,T>+Copy> Monoid for Prod<T> {
    static pure fn mempty() -> Prod<T> { Prod(One::one()) }
    pure fn mappend(&self, other: &Prod<T>) -> Prod<T> { Prod(self.repr * other.repr) }
}

impl<T: Copy> Unwrap<T> for Prod<T> {
    pure fn unwrap(&self) -> T { self.repr }
}

impl<T: cmp::Eq> Eq for Prod<T> {
    pure fn eq(&self, other: &Prod<T>) -> bool { self.repr == other.repr }
    pure fn ne(&self, other: &Prod<T>) -> bool { self.repr != other.repr }
}

pub struct Max<T> { repr: T }
pub pure fn Max<T>(val: T) -> Max<T> { Max{ repr: val } }

impl<T: Copy+Bounded+Ord> Monoid for Max<T> {
    static pure fn mempty() -> Max<T> { Max(Bounded::min_value()) }
    pure fn mappend(&self, other: &Max<T>) -> Max<T> {
        if self.repr < other.repr { *other } else { *self }
    }
}

impl<T: Copy> Unwrap<T> for Max<T> {
    pure fn unwrap(&self) -> T { self.repr }
}

impl<T: Eq> Eq for Max<T> {
    pure fn eq(&self, other: &Max<T>) -> bool { self.repr == other.repr }
    pure fn ne(&self, other: &Max<T>) -> bool { self.repr != other.repr }
}

pub struct Min<T> { repr: T }
pub pure fn Min<T>(val: T) -> Min<T> { Min { repr: val } }

impl<T: Copy+Bounded+Ord> Monoid for Min<T> {
    static pure fn mempty() -> Min<T> { Min(Bounded::max_value()) }
    pure fn mappend(&self, other: &Min<T>) -> Min<T> {
        if self.repr > other.repr { *other } else { *self }
    }
}

impl<T: Copy> Unwrap<T> for Min<T> {
    pure fn unwrap(&self) -> T { self.repr }
}


impl<T: Eq> Eq for Min<T> {
    pure fn eq(&self, other: &Min<T>) -> bool { self.repr == other.repr }
    pure fn ne(&self, other: &Min<T>) -> bool { self.repr != other.repr }
}

pub fn mconcat<T: Copy+Monoid>(v: &[T]) -> T {
    vec::foldl(Monoid::mempty(), v, |accum, elt| { elt.mappend(&accum) })
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

    fn to_sum<T: Copy, U: Copy>(tp: &(T, U)) -> (T, Sum<U>) {
        let (t, u) = *tp;
        return (t, Sum(u));
    }

    fn to_max<T: Copy, U: Copy+Ord>(tp: &(T, U)) -> (T, Max<U>) {
        let (t, u) = *tp;
        return (t, Max(u));
    }

    #[test]
    fn test_merge() {
        let arg1 = [(1, 1), (3, 1), (4, 1), (6, 1)];
        let arg2 = [(1, 2), (2, 1), (4, 1), (7, 2)];

        {
            let result = ~[(1, 3), (2, 1), (3, 1), (4, 2), (6, 1), (7, 2)];
            fail_unless!(merge(arg1.map(to_sum), arg2.map(to_sum)) == result.map(to_sum));
            fail_unless!(merge_as(arg1, arg2, Sum) == result);
        }

        {
            let result = ~[(1, 2), (2, 1), (3, 1), (4, 1), (6, 1), (7, 2)];
            fail_unless!(merge(arg1.map(to_max), arg2.map(to_max)) == result.map(to_max));
            fail_unless!(merge_as(arg1, arg2, Max) == result);
        }

        {
            let result = arg1.map(to_sum);
            fail_unless!(merge(result, []) == result);
        }

        {
            let result: ~[(int, Sum<int>)] = ~[];
            fail_unless!(merge([], []) == result);
        }
    }

    #[test]
    fn test_mergei() {
        {
            let arg = [~[(1, 1), (2, 1)], ~[(1, 2), (3, 1)], ~[(-1, 3)]];
            let result = ~[(-1, 3), (1, 3), (2, 1), (3, 1)];
            fail_unless!(mergei(arg.map(|v| v.map(to_sum))) == result.map(to_sum));
            fail_unless!(mergei_as(arg, Sum) == result);
        }

        {
            let arg = [~[(1, 1)], ~[(1, 2)], ~[(1, 3)]];
            let result = ~[(1, 6)];
            fail_unless!(mergei(arg.map(|v| v.map(to_sum))) == result.map(to_sum));
            fail_unless!(mergei_as(arg, Sum) == result);
        }

        {
            let arg = [~[], ~[], ~[]];
            let result: ~[(int, int)] = ~[];
            fail_unless!(mergei(arg.map(|v| v.map(to_sum))) == result.map(to_sum));
            fail_unless!(mergei_as(arg, Sum) == result);
        }
    }
}

