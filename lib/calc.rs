use dvec::{ DVec };
use cmp::{ Ord, Eq };
use to_bytes::{ IterBytes };
use hash::{ Hash };
use num::{ from_int };
use std::map::{ HashMap };

pub fn each_fib<T: Num Copy>(f: fn(n: &T)->bool) {
    let mut (prev, cur) = (from_int::<T>(0), from_int::<T>(1));
    loop {
        if !f(&cur) { break; }
        let next = prev + cur;
        prev = cur;
        cur  = next;
    }
}

pub fn factorial(n: uint) -> uint {
    let mut prod = 1;
    for uint::range(1, n + 1) |n| { prod *= n; }
    return prod;
}

pub fn histogram<T: Eq IterBytes Hash Const Copy>(v: &[T]) -> HashMap<T, uint> {
    let map = HashMap::<T, uint>();
    for v.each |k| {
        let val = do map.find(*k).map_default(1) |v| { *v + 1 };
        map.insert(*k, val);
    }
    return map;
}

pub fn num_of_permutations<T: Eq IterBytes Hash Copy>(hist: HashMap<T, uint>) -> uint {
    let mut sum = 0;
    let mut div = 1;
    for hist.each_value |cnt| { sum += cnt; div *= factorial(cnt); }
    return factorial(sum) / div;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_each_fib() {
        let fib = ~[ 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233 ];
        let mut calc = ~[];
        for each_fib |f: &uint| {
            if *f > fib.last() { break; }
            calc += [ *f ];
        }
        assert fib == calc;
    }

    #[test]
    fn test_factorial() {
        assert factorial(0) == 1;
        assert factorial(1) == 1;
        assert factorial(2) == 2;
        assert factorial(3) == 6;
        assert factorial(10) == 3628800;
    }

    #[test]
    fn test_histogram() {
        fn check(inp: &[uint], result: &[(uint, uint)]) {
            let mut vec = ~[];
            for histogram(inp).each |k, v| { vec += [(k, v)]; }
            let sorted = std::sort::merge_sort(|a, b| a.first() <= b.first(), vec);
            assert result == sorted;
        }
        check(&[1, 2, 3], &[(1, 1), (2, 1), (3, 1)]);
        check(&[1, 1, 1, 2, 2, 3, 3, 4], &[(1, 3), (2, 2), (3, 2), (4, 1)]);
        check(&[1, 1, 1, 2, 2, 1, 1], &[(1, 5), (2, 2)]);
        check(&[], &[]);
    }

    #[test]
    fn test_num_of_permutasions() {
        assert num_of_permutations(histogram::<uint>(&[])) == 1;
        assert num_of_permutations(histogram(&[1, 2, 3])) == 6;
        assert num_of_permutations(histogram(&[1, 1, 1, 2, 3])) == 20;
        assert num_of_permutations(histogram(&[1, 1, 1, 2, 3, 1, 1])) == 42;
    }
}
