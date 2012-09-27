use cmp::{ Ord, Eq };
use bounded::{ Bounded, max_value, min_value };

pub trait Monoid {
    static pure fn mempty() -> self;
    pure fn mappend(&self, other: &self) -> self;
}

pub trait Unwrap<T> {
    pure fn unwrap(&self) -> T;
}

pub struct Sum<T> { repr: T }
pub pure fn Sum<T>(val: T) -> Sum<T> { Sum { repr: val } }

impl<T: Num Copy> Sum<T> : Monoid {
    static pure fn mempty() -> Sum<T> { Sum(num::from_int(0)) }
    pure fn mappend(&self, other: &Sum<T>) -> Sum<T> { Sum(self.repr + other.repr) }
}

impl<T: Copy> Sum<T> : Unwrap<T> {
    pure fn unwrap(&self) -> T { self.repr }
}

impl<T: cmp::Eq> Sum<T> : cmp::Eq {
    pure fn eq(other: &Sum<T>) -> bool { self.repr == other.repr }
    pure fn ne(other: &Sum<T>) -> bool { self.repr != other.repr }
}

pub struct Prod<T> { repr: T }
pub pure fn Prod<T>(val: T) -> Prod<T> { Prod { repr: val }}

impl<T: Num Copy> Prod<T> : Monoid {
    static pure fn mempty() -> Prod<T> { Prod(num::from_int(1)) }
    pure fn mappend(&self, other: &Prod<T>) -> Prod<T> { Prod(self.repr * other.repr) }
}

impl<T: Copy> Prod<T> : Unwrap<T> {
    pure fn unwrap(&self) -> T { self.repr }
}

impl<T: cmp::Eq> Prod<T> : cmp::Eq {
    pure fn eq(other: &Prod<T>) -> bool { self.repr == other.repr }
    pure fn ne(other: &Prod<T>) -> bool { self.repr != other.repr }
}

pub struct Max<T> { repr: T }
pub pure fn Max<T>(val: T) -> Max<T> { Max{ repr: val } }

impl<T: Copy Bounded Ord> Max<T> : Monoid {
    static pure fn mempty() -> Max<T> { Max(min_value()) }
    pure fn mappend(&self, other: &Max<T>) -> Max<T> {
        if self.repr < other.repr { *other } else { *self }
    }
}

impl<T: Copy> Max<T> : Unwrap<T> {
    pure fn unwrap(&self) -> T { self.repr }
}

impl<T: Eq> Max<T> : Eq {
    pure fn eq(other: &Max<T>) -> bool { self.repr == other.repr }
    pure fn ne(other: &Max<T>) -> bool { self.repr != other.repr }
}

pub struct Min<T> { repr: T }
pub pure fn Min<T>(val: T) -> Min<T> { Min { repr: val } }

impl<T: Copy Bounded Ord> Min<T> : Monoid {
    static pure fn mempty() -> Min<T> { Min(max_value()) }
    pure fn mappend(&self, other: &Min<T>) -> Min<T> {
        if self.repr > other.repr { *other } else { *self }
    }
}

impl<T: Copy> Min<T> : Unwrap<T> {
    pure fn unwrap(&self) -> T { self.repr }
}


impl<T: Eq> Min<T> : Eq {
    pure fn eq(other: &Min<T>) -> bool { self.repr == other.repr }
    pure fn ne(other: &Min<T>) -> bool { self.repr != other.repr }
}

pub fn mconcat<T: Copy Monoid>(v: &[T]) -> T {
    vec::foldl(mempty(), v, |accum, elt| { elt.mappend(&accum) })
}

pub fn merge<T: Copy Ord, M: Copy Monoid>(vec1: &[(T, M)], vec2: &[(T, M)]) -> ~[(T, M)] {
    let mut result = ~[];
    let mut (itr1, itr2) = (vec1, vec2);
    while (itr1.is_not_empty() && itr2.is_not_empty()) {
        let ((v1, m1), (v2, m2)) = (itr1.head(), itr2.head());
        if v1 < v2 {
            result.push((v1, m1));
            itr1 = vec::view(itr1, 1u, itr1.len());
            loop;
        }
        if v2 < v1 {
            result.push((v2, m2));
            itr2 = vec::view(itr2, 1u, itr2.len());
            loop;
        }
        result.push((v1, m1.mappend(&m2)));
        itr1 = vec::view(itr1, 1u, itr1.len());
        itr2 = vec::view(itr2, 1u, itr2.len());
    }

    if itr1.is_not_empty() { result += itr1; }
    if itr2.is_not_empty() { result += itr2; }
    return result;
}

pub fn mergei<T: Copy Ord, M: Copy Monoid>(vecs: &[~[(T, M)]]) -> ~[(T, M)] {
    return match vecs.len() {
      0u => ~[],
      1u => ~[] + vecs[0],
      len  => {
        let pre  = mergei(vec::view(vecs, 0u, len / 2u));
        let post = mergei(vec::view(vecs, len / 2u, len));
        merge(pre, post)
      }
    }
}

pub fn merge_as<T: Copy Ord, U: Copy, MT, M: Copy Monoid Unwrap<MT>>
    (vec1: &[(T, U)], vec2: &[(T, U)], f: fn(U) -> M) -> ~[(T, MT)] {
    fn convert<T: Copy, U: Copy, M: Copy>(v: &[(T, U)], f: fn(U) -> M) -> ~[(T, M)] {
        do v.map |tp| { (tp.first(), f(tp.second())) }
    }
    return merge(convert(vec1, f), convert(vec2, f)).map(|tp| (tp.first(), tp.second().unwrap()));
}

pub fn mergei_as<T: Copy Ord, U: Copy, MT, M: Copy Monoid Unwrap<MT>>(vecs: &[~[(T, U)]], f: fn(U) -> M) -> ~[(T, MT)] {
    return mergei(
        vecs.map(|v| v.map(|tp| (tp.first(), f(tp.second()))))
    ).map(|tp| (tp.first(), tp.second().unwrap()));
}


#[cfg(test)]
mod tests {
    fn to_sum<T: Copy, U: Copy Num>(tp: &(T, U)) -> (T, Sum<U>) {
        let (t, u) = *tp;
        return (t, Sum(u));
    }

    fn to_max<T: Copy, U: Copy Ord>(tp: &(T, U)) -> (T, Max<U>) {
        let (t, u) = *tp;
        return (t, Max(u));
    }

    #[test]
    fn test_merge() {
        let arg1 = [(1, 1), (3, 1), (4, 1), (6, 1)];
        let arg2 = [(1, 2), (2, 1), (4, 1), (7, 2)];

        {
            let result = ~[(1, 3), (2, 1), (3, 1), (4, 2), (6, 1), (7, 2)];
            assert merge(arg1.map(to_sum), arg2.map(to_sum)) == result.map(to_sum);
            assert merge_as(arg1, arg2, Sum) == result;
        }

        {
            let result = ~[(1, 2), (2, 1), (3, 1), (4, 1), (6, 1), (7, 2)];
            assert merge(arg1.map(to_max), arg2.map(to_max)) == result.map(to_max);
            assert merge_as(arg1, arg2, Max) == result;
        }

        {
            let result = arg1.map(to_sum);
            assert merge(result, []) == result;
        }

        {
            let result: ~[(int, Sum<int>)] = ~[];
            assert merge([], []) == result;
        }
    }

    #[test]
    fn test_mergei() {
        {
            let arg = [~[(1, 1), (2, 1)], ~[(1, 2), (3, 1)], ~[(-1, 3)]];
            let result = ~[(-1, 3), (1, 3), (2, 1), (3, 1)];
            assert mergei(arg.map(|v| v.map(to_sum))) == result.map(to_sum);
            assert mergei_as(arg, Sum) == result;
        }

        {
            let arg = [~[(1, 1)], ~[(1, 2)], ~[(1, 3)]];
            let result = ~[(1, 6)];
            assert mergei(arg.map(|v| v.map(to_sum))) == result.map(to_sum);
            assert mergei_as(arg, Sum) == result;
        }

        {
            let arg = [~[], ~[], ~[]];
            let result: ~[(int, int)] = ~[];
            assert mergei(arg.map(|v| v.map(to_sum))) == result.map(to_sum);
            assert mergei_as(arg, Sum) == result;
        }
    }
}

