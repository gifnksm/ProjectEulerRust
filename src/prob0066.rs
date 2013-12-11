#[crate_type = "lib"];

extern mod extra;
extern mod math;
use std::iter;
use std::iter::Peekable;
use extra::bigint::BigUint;
use math::cont_frac;

pub static EXPECTED_ANSWER: &'static str = "661";

struct Difference<E, M, S> {
    minuend: M,
    subtrahend: Peekable<E, S>
}

impl<E, M, S: Iterator<E>> Difference<E, M, S> {
    fn new(m: M, s: S) -> Difference<E, M, S> {
        Difference { minuend: m, subtrahend: s.peekable() }
    }
}

impl<E: Eq + Ord + TotalOrd, M: Iterator<E>, S: Iterator<E>> Iterator<E> for Difference<E, M, S> {
    fn next(&mut self) -> Option<E> {
        'minuend: loop {
            match self.minuend.next() {
                None => return None,
                Some(n) => 'subtrahend: loop {
                    let cmp = match self.subtrahend.peek() {
                        None    => return Some(n),
                        Some(p) => n.cmp(p)
                    };
                    match cmp {
                        Less    => return Some(n),
                        Equal   => continue 'minuend,
                        Greater => { self.subtrahend.next(); continue 'subtrahend }
                    }
                }
            }
        }
    }
}

pub fn solve() -> ~str {
    let ns = iter::count(1u, 1);
    let sq = iter::count(1u, 1).map(|x| x*x);

    Difference::new(ns, sq)
        .take_while(|&d| d <= 1000)
        .max_by(|&d| cont_frac::solve_pel::<BigUint>(d).n0())
        .unwrap()
        .to_str()
}

#[cfg(test)]
mod test {
    mod difference_iter {
        use std::iter;
        use super::super::Difference;

        #[test]
        fn no_square_nums() {
            let ns = iter::count(1, 1);
            let sq = iter::count(1, 1).map(|x| x*x);
            let diff = Difference::new(ns, sq);
            assert_eq!(~[2, 3, 5, 6, 7, 8, 10, 11],
                       diff.take(8).to_owned_vec());
        }

        #[test]
        fn minuend_is_empty() {
            let a: ~[int] = ~[];
            let b = ~[1, 2, 3];
            let mut diff = Difference::new(a.iter(), b.iter());
            assert_eq!(~[], diff.to_owned_vec());
        }

        #[test]
        fn subtrahend_is_empty() {
            let a = ~[1, 2, 3];
            let b: ~[int] = ~[];
            let mut diff = Difference::new(a.move_iter(), b.move_iter());
            assert_eq!(~[1, 2, 3], diff.to_owned_vec());
        }
    }
}
