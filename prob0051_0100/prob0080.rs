use std::bigint::{ BigInt };

use common::arith::{ isqrt };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 80,
    answer: "40886",
    solver: solve
};

fn sqrt_newton_raphson(n: uint, precision: uint) -> ~str {
    assert!(precision >= 1);

    let n = BigInt::from_uint(n);
    let _10 = BigInt::from_uint(10);
    let mut ds = BigInt::from_uint(1);
    for (precision - 1).times { ds = ds * _10; }

    let shift = 4 * precision; // log_2 10 = 3.3... < 4
    let _1_2 = BigInt::from_uint(1) << (2 * shift);
    let mut x_1 = (BigInt::from_uint(1) << shift) / _10;
    let mut delta_2 = (_1_2 - (x_1 * x_1 * n));

    loop {
        x_1 = ((x_1 << (2 * shift)) + ((x_1 * delta_2) >> 1)) >> (2 * shift);
        delta_2 = (_1_2 - (x_1 * x_1 * n));
        if ((ds * delta_2) >> (2 * shift)).is_zero() { break; }
    }

    return ((n * x_1 * ds) >> shift).to_str();
}

fn solve() -> ~str {
    let mut total = 0;
    for uint::range(2, 101) |n| {
        let isqn = isqrt(n);
        if isqn * isqn == n { loop; }

        let sqn = sqrt_newton_raphson(n, 100);
        let sum = str::to_chars(sqn)
            .filter_mapped(|&c| char::to_digit(c, 10))
            .foldl(0u, |d, &s| d + s);
        total += sum;
    }
    return total.to_str();
}
