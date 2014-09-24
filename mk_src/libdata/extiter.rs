use std::collections::bitv::BitvSet;

pub struct Comb {
    consumed: bool,
    size: uint,
    set: BitvSet
}

impl Iterator<BitvSet> for Comb {
    fn next(&mut self) -> Option<BitvSet> {
        if self.consumed { return None }

        let result = self.set.clone();
        match self.find_change_bit() {
            None => { self.consumed = true }
            Some(n) => {
                self.set.remove(&n);
                self.set.insert(n + 1);

                let mut j = n + 2;
                for i in range(n + 2, self.size) {
                    if self.set.contains(&i) {
                        if i != j {
                            self.set.remove(&i);
                            self.set.insert(j);
                        }
                        j += 1;
                    }
                }
            }
        }
        Some(result)
    }
}

impl Comb {
    #[inline]
    pub fn new(cnt: uint, size: uint) -> Comb {
        assert!(cnt <= size);
        let mut set = BitvSet::new();
        for i in range(0, cnt) {
            set.insert(i);
        }
        Comb { consumed: false, size: size, set: set }
    }

    fn find_change_bit(&self) -> Option<uint> {
        if self.size == 0 { return None }

        for n in range(0, self.size - 1).rev() {
            if self.set.contains(&n) && !self.set.contains(&(n + 1)) {
                return Some(n)
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::Comb;

    #[test]
    fn comb() {
        fn check(cnt: uint, size: uint, expected: Vec<Vec<uint>>) {
            let actual = Comb::new(cnt, size)
                .map(|set| set.iter().collect())
                .collect::<Vec<Vec<_>>>();
            assert_eq!(actual, expected);
        }
        check(0, 4, vec![vec![]]);
        check(1, 4, vec![vec![0], vec![1], vec![2], vec![3]]);
        check(2, 4, vec![vec![0, 1], vec![0, 2], vec![0, 3],
                      vec![1, 2], vec![1, 3],
                      vec![2, 3]]);
        check(3, 4, vec![vec![0, 1, 2], vec![0, 1, 3], vec![0, 2, 3], vec![1, 2, 3]]);
        check(4, 4, vec![vec![0, 1, 2, 3]]);

        check(0, 0, vec![vec![]]);
        check(0, 1, vec![vec![]]);
        check(1, 1, vec![vec![0]]);
    }
}

#[cfg(test)]
mod bench {
    use super::Comb;
    use test::Bencher;

    #[bench]
    fn comb(bh: &mut Bencher) {
        bh.iter(|| { Comb::new(5, 10).last(); });
    }
}
