#[crate_id = "prob0103"];
#[crate_type = "rlib"];

extern crate collections;

use std::vec;
use collections::priority_queue::PriorityQueue;

pub static EXPECTED_ANSWER: &'static str = "20313839404245";

struct SSSElem {
    avg: f64,
    sss: ~[uint],
    sums: ~[uint]
}

impl Ord for SSSElem {
    #[inline(always)]
    fn lt(&self, other: &SSSElem) -> bool { self.avg >  other.avg }
    #[inline(always)]
    fn le(&self, other: &SSSElem) -> bool { self.avg >= other.avg }
    #[inline(always)]
    fn gt(&self, other: &SSSElem) -> bool { self.avg <  other.avg }
    #[inline(always)]
    fn ge(&self, other: &SSSElem) -> bool { self.avg <= other.avg }
}

impl SSSElem {
    #[inline]
    pub fn new_pair(a: uint, b: uint) -> SSSElem {
        assert!(a < b);
        SSSElem {
            avg: ((a + b) as f64) / 2.0,
            sss: ~[a, b],
            sums: ~[0, a, b, a + b]
        }
    }

    pub fn add_num(&self, n: uint) -> Option<SSSElem> {
        let mut i = 0;
        let mut j = 0;
        let len = self.sums.len();
        let mut sums = vec::with_capacity(len * 2);

        while i < len {
            assert!(j <= i);

            match self.sums[i].cmp(&(self.sums[j] + n)) {
                Equal => { return None; }
                Less => {
                    sums.push(self.sums[i]);
                    i += 1;
                }
                Greater => {
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
        let sss = vec::append_one(self.sss.to_owned(), n);
        Some(SSSElem { avg: avg, sss: sss, sums: sums })
    }

    // 6: [a, b, c, d, e, f] => (a + b + c + d) - (e + f) - 1
    // 5: [a, b, c, d, e]    => (a + b + c)     - e       - 1
    // 4: [a, b, c, d]       => (a + b + c)     - d       - 1
    // 3: [a, b, c]          => (a + b)                   - 1
    // 2: [a, b]             => (a + b)                   - 1
    #[inline]
    pub fn max_addable(&self) -> uint {
        let len   = self.sss.len();
        let add_len = len / 2 + 1;
        let sub_len = len / 2 - 1;

        let add = self.sss.slice(0, add_len).iter().fold(0, |a, &b| a + b);
        let sub = self.sss.slice(len - sub_len, len).iter().fold(0, |a, &b| a + b);
        add - sub - 1
    }

    #[inline(always)]
    pub fn each_next(&self, f: |SSSElem| -> bool) -> bool {
        if self.sss.len() == 2 {
            let (a, b) = (self.sss[0], self.sss[1]);
            if !f(SSSElem::new_pair(a, b + 1)) { return false; }
            if a == b - 1 && !f(SSSElem::new_pair(a + 1, b + 1)) { return false; }
        }

        for n in range(self.sss.last().unwrap() + 1, self.max_addable() + 1) {
            match self.add_num(n) {
                Some(x) => {
                    if !f(x) { return false; }
                }
                None => {}
            }
        }
        true
    }
}

#[inline(always)]
fn each_sss(f: |&SSSElem| -> bool) -> bool {
    let mut pq = PriorityQueue::new();
    pq.push(SSSElem::new_pair(1, 2));
    while !pq.is_empty() {
        let e = pq.pop();
        if !f(&e) { return false; }
        e.each_next(|next| {
                pq.push(next);
                true
            });
    }
    true
}

// (a, b) => SSS if a > b
// (a, b, c) => SSS if a > b > c && a + b > c
// (a, b, c, d) +> SSS if a > b > c > d && a + b > d &&
pub fn solve() -> ~str {
    let mut ans = ~"";
    each_sss(|sss| {
            if sss.sss.len() == 7 {
                ans = sss.sss.map(|&n| n.to_str()).concat();
                false
            } else {
                true
            }
        });
    ans
}
