//! [Problem 103](https://projecteuler.net/problem=103) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use std::{cmp::Ordering, collections::BinaryHeap};

/// Special sum set
#[derive(Debug)]
struct Sss {
    avg: f64,
    nums: Vec<u32>,
    sums: Vec<u32>,
}

impl PartialEq for Sss {
    fn eq(&self, other: &Sss) -> bool {
        self.avg == other.avg
    }
}
impl Eq for Sss {}

impl PartialOrd for Sss {
    fn partial_cmp(&self, other: &Sss) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Sss {
    fn cmp(&self, other: &Self) -> Ordering {
        self.avg.total_cmp(&other.avg).reverse()
    }
}

impl Sss {
    #[cfg(test)]
    fn new() -> Sss {
        Sss {
            avg: 0.0,
            nums: vec![],
            sums: vec![0],
        }
    }

    fn new_with_pair(a: u32, b: u32) -> Sss {
        assert!(a < b);
        Sss {
            avg: ((a + b) as f64) / 2.0,
            nums: vec![a, b],
            sums: vec![0, a, b, a + b],
        }
    }

    fn append_num(&self, n: u32) -> Option<Sss> {
        let mut i = 0;
        let mut j = 0;
        let len = self.sums.len();
        let mut sums = Vec::with_capacity(len * 2);

        while i < len {
            assert!(j <= i);

            match self.sums[i].cmp(&(self.sums[j] + n)) {
                Ordering::Equal => {
                    return None;
                }
                Ordering::Less => {
                    sums.push(self.sums[i]);
                    i += 1;
                }
                Ordering::Greater => {
                    sums.push(self.sums[j] + n);
                    j += 1;
                }
            }
        }
        while j < len {
            sums.push(self.sums[j] + n);
            j += 1;
        }

        let avg = (self.avg * (len as f64) + n as f64) / ((len as f64) + 1.0);
        let mut nums = self.nums.clone();
        nums.push(n);

        Some(Sss { avg, nums, sums })
    }

    fn max_addable(&self) -> u32 {
        // 6: [a, b, c, d, e, f] => (a + b + c + d) - (e + f) - 1
        // 5: [a, b, c, d, e]    => (a + b + c)     - e       - 1
        // 4: [a, b, c, d]       => (a + b + c)     - d       - 1
        // 3: [a, b, c]          => (a + b)                   - 1
        // 2: [a, b]             => (a + b)                   - 1
        let len = self.nums.len();
        let add = self.nums[..len / 2 + 1].iter().sum::<u32>();
        let sub = self.nums[(len + 1) / 2 + 1..].iter().sum::<u32>();
        add - sub - 1
    }

    fn each_next<F: FnMut(Sss)>(&self, mut f: F) {
        if self.nums.len() == 2 {
            let a = self.nums[0];
            let b = self.nums[1];
            f(Sss::new_with_pair(a, b + 1));
            if a == b - 1 {
                f(Sss::new_with_pair(a + 1, b + 1));
            }
        }

        for n in (*self.nums.last().unwrap())..(self.max_addable() + 1) {
            if let Some(sss) = self.append_num(n) {
                f(sss)
            }
        }
    }
}

struct SSSIterator {
    heap: BinaryHeap<Sss>,
}

impl SSSIterator {
    fn new() -> SSSIterator {
        let mut heap = BinaryHeap::new();
        heap.push(Sss::new_with_pair(1, 2));
        SSSIterator { heap }
    }
}

impl Iterator for SSSIterator {
    type Item = Sss;

    fn next(&mut self) -> Option<Sss> {
        self.heap.pop().map(|sss| {
            sss.each_next(|next| self.heap.push(next));
            sss
        })
    }
}

fn solve() -> String {
    let sss = SSSIterator::new().find(|sss| sss.nums.len() == 7).unwrap();
    sss.nums
        .iter()
        .map(|&n| n.to_string())
        .collect::<Vec<_>>()
        .concat()
}

common::problem!("20313839404245", solve);

#[cfg(test)]
mod tests {
    use super::Sss;
    #[test]
    fn append_num() {
        let mut sss = Sss::new();
        assert!(sss.nums.is_empty());
        sss = sss.append_num(1).unwrap();
        assert_eq!(vec![1], sss.nums);
        sss = sss.append_num(2).unwrap();
        assert_eq!(vec![1, 2], sss.nums);
        assert_eq!(None, sss.append_num(3));
        sss = sss.append_num(4).unwrap();
        assert_eq!(vec![1, 2, 4], sss.nums);
    }
}
