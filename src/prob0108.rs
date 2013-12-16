#[crate_type = "rlib"];

extern mod math;
use std::iter;
use std::iter::MultiplicativeIterator;
use math::prime;

pub static EXPECTED_ANSWER: &'static str = "180180";

// 1/x + 1/y = 1/n
// a := x - n >= 0
// b := y - n >= 0
//
// 1/(n+a) + 1/(n+b) = 1/n
// n(n+b) + n(n+a) = (n+a)(n+b)
// 2n^2 + n(a+b) = n^2 + n(a+b) + ab
// n^2 = ab

fn num_pairs(n: uint) -> uint {
    (prime::factorize(n)
     .map(|(_base, exp)| 2 * (exp as uint) + 1)
     .product() - 1) / 2 + 1
}

pub fn solve() -> ~str {
    let n = 1000;
    iter::count(1u, 1).find(|&i| num_pairs(i) > n).unwrap().to_str()
}

#[cfg(test)]
mod test {
    #[test]
    fn test_num_pairs() {
        assert_eq!(super::num_pairs(4), 3);
        assert_eq!(super::num_pairs(1260), 113);
    }
}
