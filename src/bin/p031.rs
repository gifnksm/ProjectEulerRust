#![warn(unused, bad_style,
        unnecessary_qualification, unnecessary_typecast, unused_result)]

#![feature(slicing_syntax)]

extern crate common;

use common::Solver;

fn count_ways(sum: uint, coins: &[uint]) -> uint {
    if coins.len() == 1 { return 1 }

    let mut ans = 0;
    for n in range(0, sum / coins[0] + 1) {
        let d = sum - n * coins[0];
        ans += count_ways(d, coins[1 ..]);
    }
    ans
}

fn compute(sum: uint) -> uint {
    let coins = [ 200, 100, 50, 20, 10, 5, 2, 1 ];
    count_ways(sum, coins)
}

fn solve() -> String {
    compute(200).to_string()
}

fn main() { Solver::new("73682", solve).run(); }

#[cfg(test)]
mod tests {
    #[test]
    fn four() {
        assert_eq!(3, super::compute(4));
    }
}
