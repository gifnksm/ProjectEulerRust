//! Some useful iterators.

#![warn(bad_style, missing_docs,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(slicing_syntax)]

/// An iterator that enumerates all combinations of elemnts.
///
/// The iteratee vector may contain the same elements multiple times.
pub struct CombinationOverlap<'a, T: 'a> {
    elems: &'a [T],
    idxs: Vec<uint>,
    consumed: bool
}

impl<'a, T> CombinationOverlap<'a, T> {
    /// Creates a new `CombinationOverlap` iterator
    ///
    /// # Example
    ///
    /// ```
    /// use iter::CombinationOverlap;
    /// let nums = &[1u, 2, 3];
    /// let mut it = CombinationOverlap::new(nums, 2);
    /// assert_eq!(Some(vec![1, 1]), it.next());
    /// assert_eq!(Some(vec![1, 2]), it.next());
    /// assert_eq!(Some(vec![1, 3]), it.next());
    /// assert_eq!(Some(vec![2, 2]), it.next());
    /// assert_eq!(Some(vec![2, 3]), it.next());
    /// assert_eq!(Some(vec![3, 3]), it.next());
    /// assert_eq!(None, it.next());
    /// ```
    pub fn new(elems: &'a [T], len: uint) -> CombinationOverlap<'a, T> {
        CombinationOverlap {
            elems: elems,
            idxs: Vec::from_elem(len, 0),
            consumed: false
        }
    }
}

impl<'a, T: Clone> Iterator<Vec<T>> for CombinationOverlap<'a, T> {
    fn next(&mut self) -> Option<Vec<T>> {
        if self.consumed {
            return None
        }

        let v = self.idxs.iter().map(|&i| self.elems[i].clone()).collect();

        match self.idxs.iter().rposition(|&i| i < self.elems.len() - 1) {
            Some(i) => {
                self.idxs[i] += 1;
                let v = self.idxs[i];
                for x in self.idxs[mut i + 1 ..].iter_mut() {
                    *x = v
                }
            }
            None => { self.consumed = true }
        }
        Some(v)
    }
}

/// An iterator that enumerates all permutations of elemnts.
pub struct Permutations<'a, T: 'a> {
    elems: &'a [T],
    idxs: Vec<uint>,
    cycles: Vec<uint>,
    consumed: bool
}

impl<'a, T: 'a> Permutations<'a, T> {
    /// Creates a new `Permutations` iterator
    ///
    /// # Example
    ///
    /// ```
    /// use iter::Permutations;
    /// let nums = &[1u, 2, 3];
    /// let mut it = Permutations::new(nums, 2);
    /// assert_eq!(Some((vec![1, 2], vec![3])), it.next());
    /// ```
    pub fn new(elems: &'a [T], n: uint) -> Permutations<'a, T> {
        Permutations {
            elems: elems,
            idxs: Vec::from_fn(elems.len(), |x| x),
            cycles: Vec::from_fn(n, |x| elems.len() - x),
            consumed: n > elems.len()
        }
    }
}

impl<'a, T: Clone> Iterator<(Vec<T>, Vec<T>)> for Permutations<'a, T> {
    fn next(&mut self) -> Option<(Vec<T>, Vec<T>)> {
        if self.consumed { return None }

        let n = self.cycles.len();
        let perm = self.idxs[..n].iter().map(|&i| self.elems[i].clone()).collect();
        let rest = self.idxs[n..].iter().map(|&i| self.elems[i].clone()).collect();

        if n == 0 {
            self.consumed = true;
            return Some((perm, rest));
        }

        loop {
            for i in range(0, n).rev() {
                self.cycles[i] -= 1;
                if self.cycles[i] == 0 {
                    let p = self.idxs.remove(i).unwrap();
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

#[cfg(test)]
mod tests {
    use super::{CombinationOverlap, Permutations};

    #[test]
    fn combinate_overlap() {
        let nums = &[1u, 2, 3, 4, 5];
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
        let nums = &[1u, 2, 3, 4, 5];

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
}
