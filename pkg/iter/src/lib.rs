//! Some useful iterators.

#![warn(
    bad_style,
    missing_docs,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]
#![cfg_attr(all(test, feature = "unstable"), feature(test))]

#[cfg(all(test, feature = "unstable"))]
extern crate test;

use bit_set::BitSet;
use std::{
    cmp::Ordering,
    iter::{self, Peekable},
};

/// An iterator that enumerates all combinations of bits.
pub struct BitCombination {
    consumed: bool,
    size: usize,
    set: BitSet,
}

impl Iterator for BitCombination {
    type Item = BitSet;

    fn next(&mut self) -> Option<BitSet> {
        if self.consumed {
            return None;
        }

        let result = self.set.clone();
        match self.find_change_bit() {
            None => self.consumed = true,
            Some(n) => {
                let _ = self.set.remove(n);
                let _ = self.set.insert(n + 1);

                let mut j = n + 2;
                for i in (n + 2)..self.size {
                    if self.set.contains(i) {
                        if i != j {
                            let _ = self.set.remove(i);
                            let _ = self.set.insert(j);
                        }
                        j += 1;
                    }
                }
            }
        }
        Some(result)
    }
}

impl BitCombination {
    /// Creates a new `BitCombination` iterator
    ///
    /// # Example
    ///
    /// ```
    /// use iter::BitCombination;
    /// let mut it = BitCombination::new(3, 4);
    /// assert_eq!(vec![0, 1, 2], it.next().unwrap().iter().collect::<Vec<_>>());
    /// assert_eq!(vec![0, 1, 3], it.next().unwrap().iter().collect::<Vec<_>>());
    /// assert_eq!(vec![0, 2, 3], it.next().unwrap().iter().collect::<Vec<_>>());
    /// assert_eq!(vec![1, 2, 3], it.next().unwrap().iter().collect::<Vec<_>>());
    /// assert_eq!(None, it.next());
    /// ```
    #[inline]
    pub fn new(cnt: usize, size: usize) -> BitCombination {
        assert!(cnt <= size);
        let mut set = BitSet::new();
        for i in 0..cnt {
            let _ = set.insert(i);
        }
        BitCombination {
            consumed: false,
            size,
            set,
        }
    }

    fn find_change_bit(&self) -> Option<usize> {
        if self.size == 0 {
            return None;
        }

        for n in (0..self.size - 1).rev() {
            if self.set.contains(n) && !self.set.contains(n + 1) {
                return Some(n);
            }
        }
        None
    }
}

/// An iterator that enumerates all combinations of elemnts.
///
/// The iteratee vector may contain the same elements multiple times.
pub struct CombinationOverlap<'a, T> {
    elems: &'a [T],
    idxs: Vec<usize>,
    consumed: bool,
}

impl<'a, T> CombinationOverlap<'a, T> {
    /// Creates a new `CombinationOverlap` iterator
    ///
    /// # Example
    ///
    /// ```
    /// use iter::CombinationOverlap;
    /// let nums = &[1, 2, 3];
    /// let mut it = CombinationOverlap::new(nums, 2);
    /// assert_eq!(Some(vec![1, 1]), it.next());
    /// assert_eq!(Some(vec![1, 2]), it.next());
    /// assert_eq!(Some(vec![1, 3]), it.next());
    /// assert_eq!(Some(vec![2, 2]), it.next());
    /// assert_eq!(Some(vec![2, 3]), it.next());
    /// assert_eq!(Some(vec![3, 3]), it.next());
    /// assert_eq!(None, it.next());
    /// ```
    pub fn new(elems: &'a [T], len: usize) -> CombinationOverlap<'a, T> {
        CombinationOverlap {
            elems,
            idxs: iter::repeat(0).take(len).collect(),
            consumed: false,
        }
    }
}

impl<'a, T: Clone> Iterator for CombinationOverlap<'a, T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        if self.consumed {
            return None;
        }

        let v = self.idxs.iter().map(|&i| self.elems[i].clone()).collect();

        match self.idxs.iter().rposition(|&i| i < self.elems.len() - 1) {
            Some(i) => {
                self.idxs[i] += 1;
                let v = self.idxs[i];
                for x in &mut self.idxs[i + 1..] {
                    *x = v
                }
            }
            None => self.consumed = true,
        }
        Some(v)
    }
}

/// An iterator that enumerates all permutations of elemnts.
pub struct Permutations<'a, T> {
    elems: &'a [T],
    idxs: Vec<usize>,
    cycles: Vec<usize>,
    consumed: bool,
}

impl<'a, T: 'a> Permutations<'a, T> {
    /// Creates a new `Permutations` iterator
    ///
    /// # Example
    ///
    /// ```
    /// use iter::Permutations;
    /// let nums = &[1, 2, 3];
    /// let mut it = Permutations::new(nums, 2);
    /// assert_eq!(Some((vec![1, 2], vec![3])), it.next());
    /// ```
    pub fn new(elems: &'a [T], n: usize) -> Permutations<'a, T> {
        let cycles = if n <= elems.len() {
            (0..n).map(|x| elems.len() - x).collect()
        } else {
            vec![]
        };
        Permutations {
            elems,
            idxs: (0..elems.len()).collect(),
            cycles,
            consumed: n > elems.len(),
        }
    }
}

impl<'a, T: Clone> Iterator for Permutations<'a, T> {
    type Item = (Vec<T>, Vec<T>);

    fn next(&mut self) -> Option<(Vec<T>, Vec<T>)> {
        if self.consumed {
            return None;
        }

        let n = self.cycles.len();
        let perm = self.idxs[..n]
            .iter()
            .map(|&i| self.elems[i].clone())
            .collect();
        let rest = self.idxs[n..]
            .iter()
            .map(|&i| self.elems[i].clone())
            .collect();

        if n == 0 {
            self.consumed = true;
            return Some((perm, rest));
        }

        loop {
            for i in (0..n).rev() {
                self.cycles[i] -= 1;
                if self.cycles[i] == 0 {
                    let p = self.idxs.remove(i);
                    self.idxs.push(p);
                    self.cycles[i] = self.elems.len() - i;
                    if i == 0 {
                        self.consumed = true;
                        return Some((perm, rest));
                    }
                } else {
                    let j = self.cycles[i];
                    let len = self.idxs.len();
                    let (p, q) = (self.idxs[i], self.idxs[len - j]);
                    self.idxs[i] = q;
                    self.idxs[len - j] = p;
                    return Some((perm, rest));
                }
            }
        }
    }
}

/// An iterator that enumerates elemnts that is contained in the first iterator.
pub struct Difference<M, S>
where
    M: Iterator,
    S: Iterator,
{
    minuend: M,
    subtrahend: Peekable<S>,
}

impl<E, M, S> Difference<M, S>
where
    M: Iterator<Item = E>,
    S: Iterator<Item = E>,
{
    /// Creates a new `Difference` iterator.
    ///
    /// ```rust
    /// use iter::Difference;
    ///
    /// let ints    = (1..);
    /// let squares = (1..).map(|n| n * n);
    /// let mut it = Difference::new(ints, squares);
    /// assert_eq!(Some(2), it.next()); // iterates non-square numbers
    /// assert_eq!(Some(3), it.next());
    /// assert_eq!(Some(5), it.next());
    /// assert_eq!(Some(6), it.next());
    /// assert_eq!(Some(7), it.next());
    /// assert_eq!(Some(8), it.next());
    /// assert_eq!(Some(10), it.next());
    /// ```
    pub fn new(m: M, s: S) -> Difference<M, S> {
        Difference {
            minuend: m,
            subtrahend: s.peekable(),
        }
    }
}

impl<E, M, S> Iterator for Difference<M, S>
where
    E: Eq + Ord,
    M: Iterator<Item = E>,
    S: Iterator<Item = E>,
{
    type Item = E;

    fn next(&mut self) -> Option<E> {
        'minuend: loop {
            let n = match self.minuend.next() {
                None => return None,
                Some(n) => n,
            };
            'subtrahend: loop {
                let cmp = match self.subtrahend.peek() {
                    None => return Some(n),
                    Some(p) => n.cmp(p),
                };
                match cmp {
                    Ordering::Less => return Some(n),
                    Ordering::Equal => continue 'minuend,
                    Ordering::Greater => {
                        let _ = self.subtrahend.next();
                        continue 'subtrahend;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BitCombination, CombinationOverlap, Permutations};

    #[test]
    fn bit_combination() {
        fn check(cnt: usize, size: usize, expected: Vec<Vec<usize>>) {
            let actual = BitCombination::new(cnt, size)
                .map(|set| set.iter().collect())
                .collect::<Vec<Vec<_>>>();
            assert_eq!(actual, expected);
        }

        check(0, 4, vec![vec![]]);
        check(1, 4, vec![vec![0], vec![1], vec![2], vec![3]]);
        check(
            2,
            4,
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![0, 3],
                vec![1, 2],
                vec![1, 3],
                vec![2, 3],
            ],
        );
        check(
            3,
            4,
            vec![vec![0, 1, 2], vec![0, 1, 3], vec![0, 2, 3], vec![1, 2, 3]],
        );
        check(4, 4, vec![vec![0, 1, 2, 3]]);

        check(0, 0, vec![vec![]]);
        check(0, 1, vec![vec![]]);
        check(1, 1, vec![vec![0]]);
    }

    #[test]
    fn combinate_overlap() {
        let nums = &[1, 2, 3, 4, 5];
        let mut it = CombinationOverlap::new(nums, 3);
        assert_eq!(Some(vec![1, 1, 1]), it.next());
        assert_eq!(Some(vec![1, 1, 2]), it.next());
        assert_eq!(Some(vec![1, 1, 3]), it.next());
        assert_eq!(Some(vec![1, 1, 4]), it.next());
        assert_eq!(Some(vec![1, 1, 5]), it.next());
        assert_eq!(Some(vec![1, 2, 2]), it.next());
        assert_eq!(Some(vec![1, 2, 3]), it.next());
        assert_eq!(Some(vec![1, 2, 4]), it.next());
        assert_eq!(Some(vec![1, 2, 5]), it.next());
        assert_eq!(Some(vec![1, 3, 3]), it.next());
        assert_eq!(Some(vec![1, 3, 4]), it.next());
        assert_eq!(Some(vec![1, 3, 5]), it.next());
        assert_eq!(Some(vec![1, 4, 4]), it.next());
        assert_eq!(Some(vec![1, 4, 5]), it.next());
        assert_eq!(Some(vec![1, 5, 5]), it.next());
        assert_eq!(Some(vec![2, 2, 2]), it.next());
        assert_eq!(Some(vec![2, 2, 3]), it.next());
        assert_eq!(Some(vec![2, 2, 4]), it.next());
        assert_eq!(Some(vec![2, 2, 5]), it.next());
        assert_eq!(Some(vec![2, 3, 3]), it.next());
        assert_eq!(Some(vec![2, 3, 4]), it.next());
        assert_eq!(Some(vec![2, 3, 5]), it.next());
        assert_eq!(Some(vec![2, 4, 4]), it.next());
        assert_eq!(Some(vec![2, 4, 5]), it.next());
        assert_eq!(Some(vec![2, 5, 5]), it.next());
        assert_eq!(Some(vec![3, 3, 3]), it.next());
        assert_eq!(Some(vec![3, 3, 4]), it.next());
        assert_eq!(Some(vec![3, 3, 5]), it.next());
        assert_eq!(Some(vec![3, 4, 4]), it.next());
        assert_eq!(Some(vec![3, 4, 5]), it.next());
        assert_eq!(Some(vec![3, 5, 5]), it.next());
        assert_eq!(Some(vec![4, 4, 4]), it.next());
        assert_eq!(Some(vec![4, 4, 5]), it.next());
        assert_eq!(Some(vec![4, 5, 5]), it.next());
        assert_eq!(Some(vec![5, 5, 5]), it.next());
        assert_eq!(None, it.next());

        let mut it = CombinationOverlap::new(nums, 1);
        assert_eq!(Some(vec![1]), it.next());
        assert_eq!(Some(vec![2]), it.next());
        assert_eq!(Some(vec![3]), it.next());
        assert_eq!(Some(vec![4]), it.next());
        assert_eq!(Some(vec![5]), it.next());
        assert_eq!(None, it.next());

        let mut it = CombinationOverlap::new(nums, 0);
        assert_eq!(Some(vec![]), it.next());
        assert_eq!(None, it.next());
    }

    #[test]
    fn permutation() {
        let nums = &[1, 2, 3, 4, 5];

        let mut it = Permutations::new(nums, 2);
        assert_eq!(Some((vec![1, 2], vec![3, 4, 5])), it.next());
        assert_eq!(Some((vec![1, 3], vec![2, 4, 5])), it.next());
        assert_eq!(Some((vec![1, 4], vec![2, 3, 5])), it.next());
        assert_eq!(Some((vec![1, 5], vec![2, 3, 4])), it.next());
        assert_eq!(Some((vec![2, 1], vec![3, 4, 5])), it.next());
        assert_eq!(Some((vec![2, 3], vec![1, 4, 5])), it.next());
        assert_eq!(Some((vec![2, 4], vec![1, 3, 5])), it.next());
        assert_eq!(Some((vec![2, 5], vec![1, 3, 4])), it.next());
        assert_eq!(Some((vec![3, 1], vec![2, 4, 5])), it.next());
        assert_eq!(Some((vec![3, 2], vec![1, 4, 5])), it.next());
        assert_eq!(Some((vec![3, 4], vec![1, 2, 5])), it.next());
        assert_eq!(Some((vec![3, 5], vec![1, 2, 4])), it.next());
        assert_eq!(Some((vec![4, 1], vec![2, 3, 5])), it.next());
        assert_eq!(Some((vec![4, 2], vec![1, 3, 5])), it.next());
        assert_eq!(Some((vec![4, 3], vec![1, 2, 5])), it.next());
        assert_eq!(Some((vec![4, 5], vec![1, 2, 3])), it.next());
        assert_eq!(Some((vec![5, 1], vec![2, 3, 4])), it.next());
        assert_eq!(Some((vec![5, 2], vec![1, 3, 4])), it.next());
        assert_eq!(Some((vec![5, 3], vec![1, 2, 4])), it.next());
        assert_eq!(Some((vec![5, 4], vec![1, 2, 3])), it.next());
        assert_eq!(None, it.next());

        let mut it = Permutations::new(nums, 7);
        assert_eq!(None, it.next());

        let mut it = Permutations::new(nums, 0);
        assert_eq!(Some((vec![], vec![1, 2, 3, 4, 5])), it.next());
        assert_eq!(None, it.next());
    }

    mod difference {
        use super::super::Difference;

        #[test]
        fn no_square_nums() {
            let ns = 1..;
            let sq = (1..).map(|x| x * x);
            let diff = Difference::new(ns, sq);
            assert_eq!(
                vec![2, 3, 5, 6, 7, 8, 10, 11],
                diff.take(8).collect::<Vec<_>>()
            );
        }

        #[test]
        fn minuend_is_empty() {
            let a: Vec<usize> = vec![];
            let b = vec![1, 2, 3];
            let diff = Difference::new(a.iter(), b.iter());
            assert!(diff.collect::<Vec<&usize>>().is_empty());
        }

        #[test]
        fn subtrahend_is_empty() {
            let a = vec![1, 2, 3];
            let b: Vec<usize> = vec![];
            let diff = Difference::new(a.into_iter(), b.into_iter());
            assert_eq!(vec![1, 2, 3], diff.collect::<Vec<_>>());
        }
    }
}

#[cfg(all(test, feature = "unstable"))]
mod bench {
    use super::BitCombination;
    use test::{self, Bencher};

    #[bench]
    fn comb(bh: &mut Bencher) {
        bh.iter(|| {
            let _ = test::black_box(BitCombination::new(5, 10).last());
        });
    }
}
