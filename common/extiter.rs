use core::iterator::{ Iterator, IteratorUtil };
use core::num::{ Zero, One };
use core::util;

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
    fn filter_map<'r, B>(self, f: &'r fn(A) -> Option<B>) -> FilterMapIterator<'r, A, B, Self>;
    fn windowed(self, n: uint) -> WindowedIterator<A, Self>;
    fn chain2<U: Iterator<A>>(self, other: U) -> ChainIterator<Self, U>;

    fn to_vec(self) -> ~[A];
    fn count_elem(self) -> uint;
    fn nth(self, n: uint) -> A;
    fn first(self) -> A;
    fn last(self) -> A;

    fn max_as<B: TotalOrd>(self, f: &fn(&A) -> B) -> A;
    fn min_as<B: TotalOrd>(self, f: &fn(&A) -> B) -> A;
}

impl<A, T: Iterator<A>> ExtIteratorUtil<A> for T {
    #[inline(always)]
    fn filter_map<'r, B>(self, f: &'r fn(A) -> Option<B>) -> FilterMapIterator<'r, A, B, T> {
        FilterMapIterator { iter: self, f: f }
    }

    #[inline(always)]
    fn windowed(self, n: uint) -> WindowedIterator<A, T> {
        WindowedIterator { iter: self, n: n, vs: ~[] }
    }

    #[inline(always)]
    fn chain2<U: Iterator<A>>(self, other: U) -> ChainIterator<T, U> {
        ChainIterator { a: self, b: other, flag: false }
    }

    #[inline(always)]
    fn to_vec(self) -> ~[A] {
        let mut v = ~[];
        let mut it = self;
        for it.advance() |x| { v.push(x); }
        return v;
    }

    #[inline(always)]
    fn count_elem(self) -> uint {
        let mut it = self;
        let mut cnt = 0;
        for it.advance |_| { cnt += 1; }
        return cnt;
    }

    #[inline(always)]
    fn nth(self, n: uint) -> A {
        let mut i = n;
        let mut it = self;
        loop {
            match it.next() {
                Some(x) => { if i == 0 { return x; }}
                None => { fail!("cannot get %uth element", n) }
            }
            i -= 1;
        }
    }

    #[inline(always)]
    fn first(self) -> A { self.nth(0) }

    #[inline(always)]
    fn last(self) -> A {
        let mut it = self;
        let mut elm = match it.next() {
            Some(x) => x,
            None    => fail!("last: empty iterator")
        };
        for it.advance |e| { elm = e; }
        return elm;
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

pub struct FilterMapIterator<'self, A, B, T> {
    priv iter: T,
    priv f: &'self fn(A) -> Option<B>
}

impl<'self, A, B, T: Iterator<A>> Iterator<B> for FilterMapIterator<'self, A, B, T> {
    #[inline]
    fn next(&mut self) -> Option<B> {
        loop {
            match self.iter.next() {
                None    => { return None; }
                Some(a) => {
                    match (self.f)(a) {
                        Some(b) => { return Some(b); }
                        None    => { loop; }
                    }
                }
            }
        }
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

pub struct ChainIterator<T, U> {
    priv a: T,
    priv b: U,
    priv flag: bool
}

impl<A, T: Iterator<A>, U: Iterator<A>> Iterator<A> for ChainIterator<T, U> {
    #[inline]
    fn next(&mut self) -> Option<A> {
        if self.flag {
            self.b.next()
        } else {
            match self.a.next() {
                Some(x) => return Some(x),
                _ => ()
            }
            self.flag = true;
            self.b.next()
        }
    }
}

pub trait AdditiveIterator<A> {
    fn sum(self) -> A;
}

impl<A: Add<A, A> + Zero, T: Iterator<A>> AdditiveIterator<A> for T {
    #[inline(always)]
    fn sum(self) -> A {
        let mut sum = Zero::zero::<A>();
        let mut it = self;
        for it.advance |n| { sum = sum + n; }
        return sum;
    }
}

pub trait MultiplicativeIterator<A> {
    fn prod(self) -> A;
}

impl<A: Mul<A, A> + One, T: Iterator<A>> MultiplicativeIterator<A> for T {
    #[inline(always)]
    fn prod(self) -> A {
        let mut prod = One::one::<A>();
        let mut it = self;
        for it.advance |n| { prod = prod * n; }
        return prod;
    }
}

pub trait OrderedIterator<A> {
    fn max(self) -> A;
    fn min(self) -> A;

    fn max_opt(self) -> Option<A>;
    fn min_opt(self) -> Option<A>;
}

impl<A: TotalOrd, T: Iterator<A>> OrderedIterator<A> for T {
    #[inline(always)]
    fn max(self) -> A {
        let mut it = self;
        let mut max = match it.next() {
            Some(x) => x,
            None => fail!("cannot get maximum element of empty iterator")
        };
        for it.advance |x| { if x.cmp(&max) == Greater { max = x; }}
        return max;
    }

    #[inline(always)]
    fn min(self) -> A {
        let mut it = self;
        let mut min = match it.next() {
            Some(x) => x,
            None => fail!("cannot get minimum element of empty iterator")
        };
        for it.advance |x| { if x.cmp(&min) == Less { min = x; }}
        return min;
    }

    #[inline(always)]
    fn max_opt(self) -> Option<A> {
        let mut it = self;
        let mut max = match it.next() {
            Some(x) => x,
            None => { return None; }
        };
        for it.advance |x| { if x.cmp(&max) == Greater { max = x; }}
        return Some(max);
    }

    #[inline(always)]
    fn min_opt(self) -> Option<A> {
        let mut it = self;
        let mut min = match it.next() {
            Some(x) => x,
            None => { return None; }
        };
        for it.advance |x| { if x.cmp(&min) == Less { min = x; }}
        return Some(min);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range() {
        fn gen(start: uint, end: uint, step: int) -> ~[uint] {
            let s = if step >= 0 { Plus(step as uint) } else { Minus((-step) as uint) };
            Range::new_with_step(start, end, s).to_vec()
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
        assert_eq!(Area2DIterator::new((0, 0), (1, 1), (0, 0), (3, 3)).to_vec(),
                  ~[(0, 0), (1, 1), (2, 2), (3, 3)]);
        assert_eq!(Area2DIterator::new((1, 1), (1, 1), (0, 0), (3, 3)).to_vec(),
                  ~[(1, 1), (2, 2), (3, 3)]);
        assert_eq!(Area2DIterator::new((3, 3), (1, 1), (0, 0), (3, 3)).to_vec(),
                  ~[(3, 3)]);
        assert_eq!(Area2DIterator::new((0, 0), (2, 2), (0, 0), (3, 3)).to_vec(),
                  ~[(0, 0), (2, 2)]);

        assert_eq!(Area2DIterator::new((0, 0), (0, 1), (0, 0), (3, 3)).to_vec(),
                  ~[(0, 0), (0, 1), (0, 2), (0, 3)]);
        assert_eq!(Area2DIterator::new((0, 0), (0, 1), (0, 0), (3, 5)).to_vec(),
                  ~[(0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5)]);
        assert_eq!(Area2DIterator::new((0, 0), (1, 2), (0, 0), (3, 5)).to_vec(),
                  ~[(0, 0), (1, 2), (2, 4)]);

        assert_eq!(Area2DIterator::new((3, 3), (-1, -1), (0, 0), (3, 3)).to_vec(),
                  ~[(3, 3), (2, 2), (1, 1), (0, 0)]);
        assert_eq!(Area2DIterator::new((3, 3), (-2, -2), (0, 0), (3, 3)).to_vec(),
                  ~[(3, 3), (1, 1)]);
    }

    #[test]
    fn test_fibonacci() {
        let it = Fibonacci::new::<uint>();
        let fib = ~[ 1u, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233 ];
        assert_eq!(it.take(fib.len()).to_vec(), fib);
    }

    #[test]
    fn test_triangle() {
        let it = Triangle::new();
        let tri = ~[1u, 3, 6, 10, 15, 21];
        assert_eq!(it.take(tri.len()).to_vec(), tri);
    }

    #[test]
    fn test_filter_map() {
        let it  = Range::new(0, 10).filter_map(|x| if x.is_even() { Some(x*x) } else { None });
        let ans = ~[0*0, 2*2, 4*4, 6*6, 8*8];
        assert_eq!(it.to_vec(), ans);
    }

    #[test]
    fn test_chain2() {
        use core::iterator::{ Counter };

        let xs = [0u, 1, 2, 3, 4, 5];
        let ys = [30u, 40, 50, 60];
        let expected = [0, 1, 2, 3, 4, 5, 30, 40, 50, 60];
        let mut it = xs.iter().chain2(ys.iter());
        let mut i = 0;
        for it.advance |&x: &uint| {
            assert_eq!(x, expected[i]);
            i += 1;
        }
        assert_eq!(i, expected.len());

        let ys = Counter::new(30u, 10).take(4);
        let mut it = xs.iter().transform(|&x| x).chain2(ys);
        let mut i = 0;
        for it.advance |x: uint| {
            assert_eq!(x, expected[i]);
            i += 1;
        }
        assert_eq!(i, expected.len());
    }

    #[test]
    fn test_count_elem() {
        assert_eq!(Range::new(0, 4).count_elem(), 4);
        assert_eq!(Range::new(0, 10).count_elem(), 10);
        assert_eq!(Range::new(10, 0).count_elem(), 0);
    }

    #[test]
    fn tespt_nth() {
        let v = &[0, 1, 2, 3, 4];
        for uint::range(0, v.len()) |i| {
            assert_eq!(v.iter().nth(i), &v[i]);
        }
    }

    #[test]
    #[should_fail]
    fn test_nth_fail() {
        let v = &[0, 1, 2, 3, 4];
        v.iter().nth(5);
    }

    #[test]
    fn test_max_as() {
        let v = [3, 2, 1, -1];
        assert_eq!(Range::new(0, 4).max_as(|&k| v[k]), 0);
    }

    #[test]
    fn test_min_as() {
        let v = [3, 2, 1, -1];
        assert_eq!(Range::new(0, 4).min_as(|&k| v[k]), 3);
    }


    #[test]
    fn test_sum() {
        assert_eq!(Range::new(0, 4).sum(), 6);
        assert_eq!(Range::new(0, 10).sum(), 45);
        assert_eq!(Range::new(10, 0).sum(), 0);
    }

    #[test]
    fn test_prod() {
        assert_eq!(Range::new(0, 4).prod(), 0);
        assert_eq!(Range::new(1, 5).prod(), 24);
        assert_eq!(Range::new(10, 0).prod(), 1);
    }

    #[test]
    fn test_max() {
        assert_eq!(Range::new(0, 4).max(), 3);
        assert_eq!(Range::new(0, 10).max(), 9);
        let v = ~[0, 10, 9, 2, 3, 5];
        assert_eq!(v.iter().transform(|v| *v).max(), 10);
    }

    #[test]
    #[should_fail]
    fn test_max_fail() {
        Range::new(10, 0).max();
    }

    #[test]
    fn test_min() {
        assert_eq!(Range::new(0, 4).min(), 0);
        assert_eq!(Range::new(0, 10).min(), 0);
        let v = ~[0, 10, 9, 2, 3, 5];
        assert_eq!(v.iter().transform(|v| *v).min(), 0);
    }

    #[test] #[should_fail]
    fn test_min_fail() {
        Range::new(10, 0).min();
    }
}