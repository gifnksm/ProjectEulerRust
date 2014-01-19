#[crate_id = "prob0113"];
#[crate_type = "rlib"];

use std::vec;
use std::iter::AdditiveIterator;

pub static EXPECTED_ANSWER: &'static str = "51161058134250";

fn num_increasing(len: uint) -> uint {
    let mut buf = vec::from_fn(len, |_| [0u, ..10]);

    for d in range(0, buf[0].len()) {
        buf[0][d] = 1;
    }
    for i in range(1, len) {
        let mut s = 0;
        for d in range(0, buf[i].len()).invert() {
            s += buf[i - 1][d];
            buf[i][d] = s;
        }
    }

    let sum = range(0, buf[len - 1].len())
        .map(|d| buf[len - 1][d])
        .sum();
    sum - 1 // all zero
}

fn num_decreasing(len: uint) -> uint {
    let mut buf = vec::from_fn(len, |_| [0u, ..11]); // 0, 1, 2, .., 9, A

    for d in range(0, buf[0].len()) {
        buf[0][d] = 1;
    }
    for i in range(1, len) {
        let mut s = 0;
        for d in range(0, buf[i].len()) {
            s += buf[i - 1][d];
            buf[i][d] = s;
        }
    }

    let sum = range(0, buf[len - 1].len())
        .map(|d| buf[len - 1][d])
        .sum();

    sum - len // A のみからなるものを取り除く
        - 1   // all zero
}

fn num_nonbouncy(len: uint) -> uint {
    let num_incr = num_increasing(len);
    let num_decr = num_decreasing(len);
    let num_incr_and_decr = 9 * len;
    num_incr + num_decr - num_incr_and_decr
}

pub fn solve() -> ~str {
    num_nonbouncy(100).to_str()
}

#[cfg(test)]
mod test {
    use super::num_nonbouncy;

    #[test]
    fn test_nonbouncy() {
        assert_eq!(12951,  num_nonbouncy(6));
        assert_eq!(277032, num_nonbouncy(10));
    }
}
