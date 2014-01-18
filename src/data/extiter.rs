use std::{cmp, uint};
use extra::bitv::BitvSet;

pub struct Range2D {
    cur: (int, int),
    dv: (int, int),
    cnt: uint
}

impl Range2D {
    #[inline]
    pub fn new((x0, y0): (int, int), (dx, dy): (int, int), (x_min, y_min): (int, int), (x_max, y_max): (int, int)) -> Range2D {
        if dx == 0 && dy == 0 { fail!("Range2D::new called with (dx, dy) == (0, 0)") }

        #[inline]
        fn get_cnt(p0: int, dp: int, min: int, max: int) -> uint {
            if p0 < min || max < p0 { return 0; }
            match dp.cmp(&0) {
                Equal   => uint::max_value,
                Greater => ((max + 1 - p0) / dp) as uint,
                Less    => ((p0 + 1 - min) / (-dp)) as uint
            }
        }

        Range2D {
            cur: (x0, y0),
            dv: (dx, dy),
            cnt: cmp::min(get_cnt(x0, dx, x_min, x_max), get_cnt(y0, dy, y_min, y_max))
        }
    }

    #[inline]
    pub fn new_from_matrix(start: (int, int), dv: (int, int), (w, h): (int, int)) -> Range2D {
        assert!(w > 0 && h > 0);
        Range2D::new(start, dv, (0, 0), (w - 1, h - 1))
    }
}

impl Iterator<(int, int)> for Range2D {
    #[inline(always)]
    fn next(&mut self) -> Option<(int, int)> {
        if self.cnt <= 0 { return None }
        self.cnt -= 1;
        let ((x, y), (dx, dy)) = (self.cur, self.dv);
        self.cur = (x + dx, y + dy);
        Some((x, y))
    }
}

pub struct Comb {
    priv consumed: bool,
    priv size: uint,
    priv set: BitvSet
}

impl Iterator<BitvSet> for Comb {
    fn next(&mut self) -> Option<BitvSet> {
        if !self.consumed {
            self.consumed = true;
            return Some(self.set.clone());
        }

        let mut n = self.size;
        loop {
            match range(0, n).invert().find(|i| self.set.contains(i)) {
                None => return None,
                Some(x) => {
                    if x < self.size - 1 && !self.set.contains(&(x + 1)) {
                        n = x;
                        break;
                    }
                    n = x;
                }
            }
        }
        self.set.remove(&n);
        self.set.insert(n + 1);
        let mut j = n + 2;
        for i in range(n + 2, self.size) {
            if self.set.contains(&i) {
                self.set.remove(&i);
                self.set.insert(j);
                j += 1;
            }
        }
        Some(self.set.clone())
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
}

#[cfg(test)]
mod test {
    use super::{Range2D, Comb};

    #[test]
    fn test_area2d() {
        let vs = Range2D::new((0, 0), (1, 1), (0, 0), (3, 3)).to_owned_vec();
        assert_eq!(vs, ~[(0, 0), (1, 1), (2, 2), (3, 3)]);

        let vs = Range2D::new((1, 1), (1, 1), (0, 0), (3, 3)).to_owned_vec();
        assert_eq!(vs, ~[(1, 1), (2, 2), (3, 3)]);

        let vs = Range2D::new((3, 3), (1, 1), (0, 0), (3, 3)).to_owned_vec();
        assert_eq!(vs, ~[(3, 3)]);

        let vs = Range2D::new((0, 0), (2, 2), (0, 0), (3, 3)).to_owned_vec();
        assert_eq!(vs, ~[(0, 0), (2, 2)]);

        let vs = Range2D::new((0, 0), (0, 1), (0, 0), (3, 3)).to_owned_vec();
        assert_eq!(vs, ~[(0, 0), (0, 1), (0, 2), (0, 3)]);

        let vs = Range2D::new((0, 0), (0, 1), (0, 0), (3, 5)).to_owned_vec();
        assert_eq!(vs, ~[(0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5)]);

        let vs = Range2D::new((0, 0), (1, 2), (0, 0), (3, 5)).to_owned_vec();
        assert_eq!(vs, ~[(0, 0), (1, 2), (2, 4)]);

        let vs = Range2D::new((3, 3), (-1, -1), (0, 0), (3, 3)).to_owned_vec();
        assert_eq!(vs, ~[(3, 3), (2, 2), (1, 1), (0, 0)]);

        let vs = Range2D::new((3, 3), (-2, -2), (0, 0), (3, 3)).to_owned_vec();
        assert_eq!(vs, ~[(3, 3), (1, 1)]);
    }

    #[test]
    fn test_comb_iterator() {
        fn check(cnt: uint, size: uint, expected: ~[~[uint]]) {
            let actual = Comb::new(cnt, size)
                .map(|set| set.iter().to_owned_vec())
                .to_owned_vec();
            assert_eq!(actual, expected);
        }
        check(0, 4, ~[~[]]);
        check(1, 4, ~[~[0], ~[1], ~[2], ~[3]]);
        check(2, 4, ~[~[0, 1], ~[0, 2], ~[0, 3],
                      ~[1, 2], ~[1, 3],
                      ~[2, 3]]);
        check(3, 4, ~[~[0, 1, 2], ~[0, 1, 3], ~[0, 2, 3], ~[1, 2, 3]]);
        check(4, 4, ~[~[0, 1, 2, 3]]);

        check(0, 0, ~[~[]]);
        check(0, 1, ~[~[]]);
        check(1, 1, ~[~[0]]);
    }
}
