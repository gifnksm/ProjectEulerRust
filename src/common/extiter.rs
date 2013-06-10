use std::iterator::{Iterator, IteratorUtil};
use std::num::{Zero, One};
use std::{util, uint};

pub enum Step<T> { Plus(T), Minus(T) }

impl<T> Step<T> {
    fn ref_abs<'a>(&'a self) -> &'a T {
        match *self {
            Plus(ref s) => s,
            Minus(ref s) => s
        }
    }
}


pub struct Range<T> {
    cur: T,
    step: Step<T>,
    cnt: T,
}

impl<T: Integer> Range<T> {
    pub fn new_with_step(start: T, stop: T, step: Step<T>) -> Range<T> {
        if step.ref_abs().is_zero() { fail!("Range::new() called with step == 0"); }

        let mut cnt = Zero::zero();
        match step {
            Plus(ref abs_step) if start < stop => {
                let diff = (stop - start);
                cnt = diff / *abs_step;
                if !diff.is_multiple_of(abs_step) { cnt = cnt + One::one(); }
            }
            Minus(ref abs_step) if start > stop => {
                let diff = (start - stop);
                cnt = diff / *abs_step;
                if !diff.is_multiple_of(abs_step) { cnt = cnt + One::one(); }
            }
            _ => { }
        }
        return Range { cur: start, cnt: cnt, step: step };
    }

    pub fn new(start: T, stop: T) -> Range<T> {
        Range::new_with_step(start, stop, Plus(One::one()))
    }
    pub fn new_rev(start: T, stop: T) -> Range<T> {
        Range::new_with_step(start, stop, Minus(One::one()))
    }
}

impl<T: Integer> Iterator<T> for Range<T> {
    #[inline(always)]
    fn next(&mut self) -> Option<T> {
        if self.cnt <= Zero::zero() { return None; }
        self.cnt = self.cnt - One::one();

        match self.step {
            Plus(ref abs_step) => {
                let mut val = self.cur + *abs_step;
                util::swap(&mut val, &mut self.cur);
                return Some(val);
            }
            Minus(ref abs_step) => {
                let mut val = self.cur - *abs_step;
                util::swap(&mut val, &mut self.cur);
                return Some(val);
            }
        }
    }
}


struct Area2DIterator {
    cur: (int, int),
    dv: (int, int),
    cnt: uint
}

impl Area2DIterator {
    #[inline(always)]
    pub fn new((x0, y0): (int, int), (dx, dy): (int, int), (x_min, y_min): (int, int), (x_max, y_max): (int, int)) -> Area2DIterator {
        if dx == 0 && dy == 0 { fail!("Area2DIterator::new called with (dx, dy) == (0, 0)") }

        #[inline(always)]
        fn get_cnt(p0: int, dp: int, min: int, max: int) -> uint {
            if p0 < min || max < p0 { return 0; }
            return match dp.cmp(&0) {
                Equal   => uint::max_value,
                Greater => ((max + 1 - p0) / dp) as uint,
                Less    => (p0 + 1 - min) / (-dp) as uint
            };
        }

        Area2DIterator {
            cur: (x0, y0),
            dv: (dx, dy),
            cnt: uint::min(get_cnt(x0, dx, x_min, x_max), get_cnt(y0, dy, y_min, y_max))
        }
    }

    #[inline(always)]
    pub fn new_from_matrix(start: (int, int), dv: (int, int), (w, h): (int, int)) -> Area2DIterator {
        assert!(w > 0 && h > 0);
        Area2DIterator::new(start, dv, (0, 0), (w - 1, h - 1))
    }
}

impl Iterator<(int, int)> for Area2DIterator {
    #[inline(always)]
    fn next(&mut self) -> Option<(int, int)> {
        if self.cnt <= 0 { return None; }
        self.cnt -= 1;
        let ((x, y), (dx, dy)) = (self.cur, self.dv);
        self.cur = (x + dx, y + dy);
        return Some((x, y));
    }
}


pub struct Fibonacci<T> {
    prev: T,
    cur: T
}

impl<T: Zero + One> Fibonacci<T> {
    pub fn new() -> Fibonacci<T> { Fibonacci { prev: Zero::zero(), cur: One::one() } }
}

impl<T: Add<T,T> + Clone> Iterator<T> for Fibonacci<T> {
    #[inline(always)]
    fn next(&mut self) -> Option<T> {
        let next = self.prev + self.cur;
        let cur  = self.cur.clone();
        self.prev = cur.clone();
        self.cur  = next;
        return Some(cur);
    }
}

pub struct Triangle {
    idx: uint,
    cur: uint
}

impl Triangle {
    pub fn new() -> Triangle { Triangle { idx: 1, cur: 1 } }
}

impl Iterator<uint> for Triangle {
    #[inline(always)]
    fn next(&mut self) -> Option<uint> {
        let cur = self.cur;
        self.idx += 1;
        self.cur += self.idx;
        return Some(cur);
    }
}



pub trait ExtIteratorUtil<A> {
    fn windowed(self, n: uint) -> WindowedIterator<A, Self>;

    fn max_as<B: TotalOrd>(self, f: &fn(&A) -> B) -> A;
    fn min_as<B: TotalOrd>(self, f: &fn(&A) -> B) -> A;
}

impl<A, T: Iterator<A>> ExtIteratorUtil<A> for T {
    #[inline(always)]
    fn windowed(self, n: uint) -> WindowedIterator<A, T> {
        WindowedIterator { iter: self, n: n, vs: ~[] }
    }

    #[inline(always)]
    fn max_as<B: TotalOrd>(self, f: &fn(&A) -> B) -> A {
        let mut it = self;
        let mut (max_val, max_key) = match it.next() {
            Some(x) => (f(&x), x),
            None => fail!("cannot get maximum element of empty iterator")
        };
        for it.advance |key| {
            let val = f(&key);
            if val.cmp(&max_val) == Greater {
                max_val = val;
                max_key = key;
            }
        }
        return max_key;
    }

    #[inline(always)]
    fn min_as<B: TotalOrd>(self, f: &fn(&A) -> B) -> A {
        let mut it = self;
        let mut (min_val, min_key) = match it.next() {
            Some(x) => (f(&x), x),
            None => fail!("cannot get minimum element of empty iterator")
        };
        for it.advance |key| {
            let val = f(&key);
            if val.cmp(&min_val) == Less {
                min_val = val;
                min_key = key;
            }
        }
        return min_key;
    }
}

pub struct WindowedIterator<A, T> {
    priv iter: T,
    priv n: uint,
    priv vs: ~[A]
}

impl<'self, A: Clone, T: Iterator<A>> Iterator<~[A]> for WindowedIterator<A, T> {
    #[inline]
    fn next(&mut self) -> Option<~[A]> {
        if self.vs.len() == self.n {
            self.vs.shift();
        }
        while self.vs.len() < self.n {
            match self.iter.next() {
                Some(x) => { self.vs.push(x); }
                None    => { return None; }
            }
        }
        return Some(self.vs.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::uint;

    #[test]
    fn test_range() {
        fn gen(start: uint, end: uint, step: int) -> ~[uint] {
            let s = if step >= 0 { Plus(step as uint) } else { Minus((-step) as uint) };
            Range::new_with_step(start, end, s).collect()
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
    fn test_area2d() {
        let vs: ~[(int, int)] = Area2DIterator::new((0, 0), (1, 1), (0, 0), (3, 3)).collect();
        assert_eq!(vs, ~[(0, 0), (1, 1), (2, 2), (3, 3)]);

        let vs: ~[(int, int)] = Area2DIterator::new((1, 1), (1, 1), (0, 0), (3, 3)).collect();
        assert_eq!(vs, ~[(1, 1), (2, 2), (3, 3)]);

        let vs: ~[(int, int)] = Area2DIterator::new((3, 3), (1, 1), (0, 0), (3, 3)).collect();
        assert_eq!(vs, ~[(3, 3)]);

        let vs: ~[(int, int)] = Area2DIterator::new((0, 0), (2, 2), (0, 0), (3, 3)).collect();
        assert_eq!(vs, ~[(0, 0), (2, 2)]);

        let vs: ~[(int, int)] = Area2DIterator::new((0, 0), (0, 1), (0, 0), (3, 3)).collect();
        assert_eq!(vs, ~[(0, 0), (0, 1), (0, 2), (0, 3)]);

        let vs: ~[(int, int)] = Area2DIterator::new((0, 0), (0, 1), (0, 0), (3, 5)).collect();
        assert_eq!(vs, ~[(0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5)]);

        let vs: ~[(int, int)] = Area2DIterator::new((0, 0), (1, 2), (0, 0), (3, 5)).collect();
        assert_eq!(vs, ~[(0, 0), (1, 2), (2, 4)]);

        let vs: ~[(int, int)] = Area2DIterator::new((3, 3), (-1, -1), (0, 0), (3, 3)).collect();
        assert_eq!(vs, ~[(3, 3), (2, 2), (1, 1), (0, 0)]);

        let vs: ~[(int, int)] = Area2DIterator::new((3, 3), (-2, -2), (0, 0), (3, 3)).collect();
        assert_eq!(vs, ~[(3, 3), (1, 1)]);
    }

    #[test]
    fn test_fibonacci() {
        let it = Fibonacci::new::<uint>();
        let fib = ~[ 1u, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233 ];
        let gen: ~[uint] = it.take(fib.len()).collect();
        assert_eq!(gen, fib);
    }

    #[test]
    fn test_triangle() {
        let it = Triangle::new();
        let tri = ~[1u, 3, 6, 10, 15, 21];
        let gen: ~[uint] = it.take(tri.len()).collect();
        assert_eq!(gen, tri);
    }
}