#![crate_name = "prob0103"]
#![crate_type = "rlib"]


use std::collections::priority_queue::PriorityQueue;

pub static EXPECTED_ANSWER: &'static str = "20313839404245";

struct SSSElem {
    avg: f64,
    sss: Vec<uint>,
    sums: Vec<uint>
}

impl PartialEq for SSSElem {
    fn eq(&self, other: &SSSElem) -> bool { self.avg == other.avg }
}
impl Eq for SSSElem {}

impl PartialOrd for SSSElem {
    fn partial_cmp(&self, other: &SSSElem) -> Option<Ordering> {
        match self.avg.partial_cmp(&other.avg) {
            Some(Less)    => Some(Greater),
            Some(Equal)   => Some(Equal),
            Some(Greater) => Some(Less),
            None          => None
        }
    }
}

impl Ord for SSSElem {
    fn cmp(&self, other: &SSSElem) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl SSSElem {
    #[inline]
    pub fn new_pair(a: uint, b: uint) -> SSSElem {
        assert!(a < b);
        SSSElem {
            avg: ((a + b) as f64) / 2.0,
            sss: vec![a, b],
            sums: vec![0, a, b, a + b]
        }
    }

    pub fn add_num(&self, n: uint) -> Option<SSSElem> {
        let mut i = 0;
        let mut j = 0;
        let len = self.sums.len();
        let mut sums = Vec::with_capacity(len * 2);

        while i < len {
            assert!(j <= i);

            match self.sums[i].cmp(&(self.sums[j] + n)) {
                Equal   => { return None; }
                Less    => { sums.push(self.sums[i]);     i += 1; }
                Greater => { sums.push(self.sums[j] + n); j += 1; }
            }
        }

        while j < len {
            sums.push(self.sums[j] + n);
            j += 1;
        }

        let avg = (self.avg * (len as f64) + n as f64) / ((len as f64) + 1.0);
        let sss = self.sss.clone().append_one(n);
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
    loop {
        match pq.pop() {
            Some(e) => {
                if !f(&e) { return false; }
                e.each_next(|next| {
                    pq.push(next);
                    true
                });
            }
            None => return true
        }
    }
}

// (a, b) => SSS if a > b
// (a, b, c) => SSS if a > b > c && a + b > c
// (a, b, c, d) +> SSS if a > b > c > d && a + b > d &&
pub fn solve() -> String {
    let mut ans = "".to_string();
    each_sss(|sss| {
            if sss.sss.len() == 7 {
                ans = sss.sss.iter().map(|&n| n.to_string()).collect::<Vec<String>>().concat();
                false
            } else {
                true
            }
        });
    ans
}
