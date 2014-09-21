#![warn(unused, bad_style,
        unnecessary_qualification, unnecessary_typecast, unused_result)]

extern crate common;

use common::Solver;

fn sum_of_square(n: uint) -> uint { n * (n + 1) * (2 * n + 1) / 6 }
fn sum_of_seq(n: uint) -> uint { n * (n + 1) / 2 }
fn square_of_sum(n: uint) -> uint {
    let s = sum_of_seq(n);
    s * s
}

fn compute(n: uint) -> uint { square_of_sum(n) - sum_of_square(n) }

fn solve() -> String { compute(100).to_string() }

fn main() { Solver::new("25164150", solve).run(); }

#[cfg(test)]
mod tests {
    #[test]
    fn sofs_ten() {
        assert_eq!(2640 , super::compute(10));
    }
}
