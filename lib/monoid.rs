trait Monoid {
    static pure fn mempty() -> self;
    pure fn mappend(&&other: self) -> self;
}

enum Sum<T> = T;
impl<T: Num> Sum<T> : Monoid {
    static pure fn mempty() -> Sum<T> { Sum(num::from_int(0)) }
    pure fn mappend(other: Sum<T>) -> Sum<T> { Sum(self.add(*other)) }
}

impl<T: cmp::Eq> Sum<T> : cmp::Eq {
    pure fn eq(&&other: Sum<T>) -> bool { *self == *other }
    pure fn ne(&&other: Sum<T>) -> bool { *self != *other }
}

enum Prod<T> = T;
impl <T: Num> Prod<T>: Monoid {
    static pure fn mempty() -> Prod<T> { Prod(num::from_int(1)) }
    pure fn mappend(other: Prod<T>) -> Prod<T> { Prod(self.mul(*other)) }
}

impl<T: cmp::Eq> Prod<T> : cmp::Eq {
    pure fn eq(&&other: Prod<T>) -> bool { *self == *other }
    pure fn ne(&&other: Prod<T>) -> bool { *self != *other }
}

fn mconcat<T: Copy Monoid>(v: &[T]) -> T {
    vec::foldl(mempty(), v, |accum, elt| { elt.mappend(accum) })
}

fn merge<T: Copy cmp::Ord, M: Copy Monoid>(vec1: &[(T, M)], vec2: &[(T, M)]) -> ~[(T, M)] {
    let mut result = ~[];
    let mut (itr1, itr2) = (vec1, vec2);
    while (itr1.is_not_empty() && itr2.is_not_empty()) {
        let ((v1, m1), (v2, m2)) = (itr1.head(), itr2.head());
        if v1.lt(v2) {
            vec::push(result, (v1, m1));
            itr1 = vec::view(itr1, 1u, itr1.len());
            loop;
        }
        if v2.lt(v1) {
            vec::push(result, (v2, m2));
            itr2 = vec::view(itr2, 1u, itr2.len());
            loop;
        }
        vec::push(result, (v1, m1.mappend(m2)));
        itr1 = vec::view(itr1, 1u, itr1.len());
        itr2 = vec::view(itr2, 1u, itr2.len());
    }

    if itr1.is_not_empty() { result += itr1; }
    if itr2.is_not_empty() { result += itr2; }
    return result;
}

fn mergei<T: Copy cmp::Ord, M: Copy Monoid>(vecs: &[~[(T, M)]]) -> ~[(T, M)] {
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

#[cfg(test)]
mod tests {
    fn to_sum<T: Copy, U: Copy Num>(tp: (T, U)) -> (T, Sum<U>) {
        let (t, u) = tp;
        return (t, Sum(u));
    }

    enum Max<T> = T;
    impl <T: Copy cmp::Ord Num> Max<T>: Monoid {
        static pure fn mempty() -> Max<T> {
            let neg_max = num::from_int::<T>(int::min_value);
            let zero    = num::from_int(0);
            if neg_max < zero {
                Max(neg_max)
            } else {
                Max(zero)
            }
        }
        pure fn mappend(&&other: Max<T>) -> Max<T> { if self.lt(*other) { other } else { self } }
    }
    impl <T: cmp::Eq> Max<T>: cmp::Eq {
        pure fn eq(&&other: Max<T>) -> bool { *self == *other }
        pure fn ne(&&other: Max<T>) -> bool { *self != *other }
    }
    fn to_max<T: Copy, U: Copy cmp::Ord>(tp: (T, U)) -> (T, Max<U>) {
        let (t, u) = tp;
        return (t, Max(u));
    }

    #[test]
    fn test_merge() {
        let arg1 = [(1, 1), (3, 1), (4, 1), (6, 1)];
        let arg2 = [(1, 2), (2, 1), (4, 1), (7, 2)];

        {
            let result = [(1, 3), (2, 1), (3, 1), (4, 2), (6, 1), (7, 2)].map(to_sum);
            assert merge(arg1.map(to_sum), arg2.map(to_sum)) == result;
        }

        {
            let result = [(1, 2), (2, 1), (3, 1), (4, 1), (6, 1), (7, 2)].map(to_max);
            assert merge(arg1.map(to_max), arg2.map(to_max)) == result;
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
            let arg = [~[to_sum((1, 1)), to_sum((2, 1))], ~[to_sum((1, 2)), to_sum((3, 1))], ~[to_sum((-1, 3))]];
            let result = [(-1, 3), (1, 3), (2, 1), (3, 1)].map(to_sum);
            assert mergei(arg) == result;
        }

        {
            let arg = [~[to_sum((1, 1))], ~[to_sum((1, 2))], ~[to_sum((1, 3))]];
            let result: ~[(int, Sum<int>)] = ~[to_sum((1, 6))];
            assert mergei(arg) == result;
        }

        {
            let result: ~[(int, Sum<int>)] = ~[];
            assert mergei([~[], ~[], ~[]]) == result;
        }
    }
}

