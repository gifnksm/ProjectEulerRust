use core::iterator::{ Iterator, IteratorUtil };
use core::num::{ Zero, One };

pub struct UintRange {
    cur: uint,
    cnt: uint,
    step: int,
}

impl UintRange {
    pub fn new(start: uint, stop: uint, step: int) -> UintRange {
        if step == 0 {
            fail!("UintRange::new called with step == 0");
        }

        let mut cnt = 0;
        if step > 0 && start < stop {
            let diff = (stop - start);
            cnt = diff / (step as uint);
            if diff % (step as uint) != 0 { cnt += 1; }
        }
        if step < 0 && start > stop {
            let diff = (start - stop);
            cnt = diff / ((-step) as uint);
            if diff % ((-step) as uint) != 0 { cnt += 1; }
        }
        UintRange { cur: start, cnt: cnt, step: step }
    }
}

pub fn uint_range(start: uint, stop: uint) -> UintRange {
    UintRange::new(start, stop, 1)
}

impl Iterator<uint> for UintRange {
    fn next(&mut self) -> Option<uint> {
        if self.cnt == 0 { return None; }

        let val = self.cur;

        match self.step.cmp(&0) {
            Greater => {
                self.cnt -= 1;
                self.cur += (self.step as uint);
                return Some(val);
            },
            Less => {
                self.cnt -= 1;
                self.cur -= ((- self.step) as uint);
                return Some(val);
            },
            Equal => { fail!() }
        }
    }
}

pub struct Fibonacci<T> {
    prev: T,
    cur: T
}

impl<T: Zero + One> Fibonacci<T> {
    pub fn new() -> Fibonacci<T> { Fibonacci { prev: Zero::zero(), cur: One::one() } }
}

// Copy must be Clone
impl<T: Add<T,T> + Copy> Iterator<T> for Fibonacci<T> {
    fn next(&mut self) -> Option<T> {
        let next = self.prev + self.cur;
        // let cur  = self.cur.clone();
        let cur  = self.cur;
        // self.prev = cur.clone();
        self.prev = cur;
        self.cur  = next;
        // return Some(cur);
        return Some(cur);
    }
}

pub struct Triangle {
    idx: uint,
    cur:  uint
}

impl Triangle {
    pub fn new() -> Triangle { Triangle { idx: 1, cur: 1 } }
}

impl Iterator<uint> for Triangle {
    fn next(&mut self) -> Option<uint> {
        let cur = self.cur;
        self.idx += 1;
        self.cur += self.idx;
        return Some(cur);
    }
}

pub fn count_elem<T, IT: Iterator<T>>(mut it: IT) -> uint {
    let mut cnt = 0;
    for it.advance |_| { cnt += 1; }
    return cnt;
}

pub fn nth<T, IT: Iterator<T>>(mut it: IT, n: uint) -> T {
    let mut i = n;
    loop {
        match it.next() {
            Some(x) => { if i == 0 { return x; } }
            None => { fail!("cannot get %uth element", n); }
        }
        i -= 1;
    }
}

pub fn sum<T: Add<T, T> + Zero, IT: Iterator<T>>(mut it: IT) -> T {
    let mut sum = Zero::zero::<T>();
    for it.advance |n| { sum = sum + n; }
    return sum;
}

pub fn max<T: TotalOrd, IT: Iterator<T>>(mut it: IT) -> T {
    let mut max = match it.next() {
        Some(x) => x,
        None => fail!()
    };
    for it.advance |x| { if x.cmp(&max) == Greater { max = x; }}
    return max;
}

pub fn min<T: TotalOrd, IT: Iterator<T>>(mut it: IT) -> T {
    let mut min = match it.next() {
        Some(x) => x,
        None => fail!()
    };
    for it.advance |x| { if x.cmp(&min) == Less { min = x; }}
    return min;
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::extvec;

    #[test]
    fn test_uint_range() {
        fn gen(start: uint, end: uint, step: int) -> ~[uint] {
            extvec::from_iter(UintRange::new(start, end, step))
        }
        assert_eq!(gen(0, 3, 1), ~[0, 1, 2]);
        assert_eq!(gen(13, 10, -1), ~[13, 12, 11]);
        assert_eq!(gen(20, 26, 2), ~[20, 22, 24]);
        assert_eq!(gen(36, 30, -2), ~[36, 34, 32]);
        assert_eq!(gen(uint::max_value - 2, uint::max_value, 2),
                   ~[uint::max_value - 2]);
        assert_eq!(gen(uint::max_value - 3, uint::max_value, 2),
                   ~[uint::max_value - 3, uint::max_value - 1]);
        assert_eq!(gen(uint::min_value + 2, uint::min_value, -2),
                   ~[uint::min_value + 2]);
        assert_eq!(gen(uint::min_value + 3, uint::min_value, -2),
                   ~[uint::min_value + 3, uint::min_value + 1]);
    }

    #[test]
    fn test_fibonacci() {
        let it = Fibonacci::new::<uint>();
        let fib = ~[ 1u, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233 ];
        assert_eq!(extvec::from_iter(it.take(fib.len())), fib);
    }

    #[test]
    fn test_triangle() {
        let it = Triangle::new();
        let tri = ~[1u, 3, 6, 10, 15, 21];
        assert_eq!(extvec::from_iter(it.take(tri.len())), tri);
    }

    #[test]
    fn test_count_elem() {
        assert_eq!(count_elem(uint_range(0, 4)), 4);
        assert_eq!(count_elem(uint_range(0, 10)), 10);
        assert_eq!(count_elem(uint_range(10, 0)), 0);
    }

    #[test]
    fn test_nth() {
        let v = &[0, 1, 2, 3, 4];
        for uint::range(0, v.len()) |i| {
            assert_eq!(nth(v.iter(), i), &v[i]);
        }
    }

    #[test]
    #[should_fail]
    fn test_nth_fail() {
        let v = &[0, 1, 2, 3, 4];
        nth(v.iter(), 5);
    }

    #[test]
    fn test_sum_uint() {
        assert_eq!(sum_uint(uint_range(0, 4)), 6);
        assert_eq!(sum_uint(uint_range(0, 10)), 45);
        assert_eq!(sum_uint(uint_range(10, 0)), 0);
    }

    #[test]
    fn test_max() {
        assert_eq!(max(uint_range(0, 4)), 3);
        assert_eq!(max(uint_range(0, 10)), 9);
        let v = ~[0, 10, 9, 2, 3, 5];
        assert_eq!(max(v.iter().transform(|v| *v)), 10);
    }

    #[test]
    #[should_fail]
    fn test_max_fail() {
        assert_eq!(max(uint_range(10, 0)), 0);
    }

    #[test]
    fn test_min() {
        assert_eq!(min(uint_range(0, 4)), 0);
        assert_eq!(min(uint_range(0, 10)), 0);
        let v = ~[0, 10, 9, 2, 3, 5];
        assert_eq!(min(v.iter().transform(|v| *v)), 0);
    }

    #[test] #[should_fail]
    fn test_min_fail() {
        min(uint_range(10, 0));
    }
}