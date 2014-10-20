#![feature(slicing_syntax)]

pub struct CombinationOverlap<'a, T: 'a> {
    elems: &'a [T],
    idxs: Vec<uint>,
    consumed: bool
}

impl<'a, T> CombinationOverlap<'a, T> {
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

#[cfg(test)]
mod tests {
    use super::CombinationOverlap;

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
        assert_eq!(None, it.next())

        let mut it = CombinationOverlap::new(nums, 0);
        assert_eq!(Some(vec![]), it.next());
        assert_eq!(None, it.next())
    }
}
