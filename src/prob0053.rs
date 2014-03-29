#![crate_id = "prob0053"]
#![crate_id = "prob0053"]
#![crate_type = "rlib"]
#![crate_type = "rlib"]

pub static EXPECTED_ANSWER: &'static str = "4075";

// nCr-1 = r/(n-r+1) nCr!
// nCr = n/(n-r) n-1Cr
// nC(r+1) = (n-r)/(r+1) nCr
pub fn solve() -> ~str {
    let limit = 1000000;

    let mut r = 0;
    let mut c = 1;
    let mut cnt = 0;
    for n in range(1, 101) {
        c = c * n / (n - r); // nCr
        if c < limit {
            while c < limit {
                if r == (n + 1) / 2 { break }
                c = c * (n - r) / (r + 1);
                r += 1;
            }
            if c < limit { continue }
        } else {
            while c * r / (n - r + 1) >= limit {
                c = c * r / (n - r + 1);
                r -= 1;
            }
        }
        cnt += ((n - r) - r) + 1;
    }

    cnt.to_str()
}
