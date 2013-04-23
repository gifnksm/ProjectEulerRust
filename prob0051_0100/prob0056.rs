use std::bigint::{ BigUint };

use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 56,
    answer: "972",
    solver: solve
};

fn digit_sum(n: uint) -> uint {
    let mut sum = 0;
    let mut d = n;
    while d > 0 {
        sum += d % 10;
        d = d / 10;
    }
    return sum;
}

fn solve() -> ~str {
    let ten = BigUint::from_uint(1000000000);

    let mut max = 0u;
    for uint::range(1, 100) |a| {
        let mut n = BigUint::from_uint(a);
        for 100.times {
            n = n * BigUint::from_uint(a);
            let mut sum = 0;
            let mut (d, m) = n.div_mod(&ten);
            sum += digit_sum(m.to_uint());
            while !d.is_zero() {
                let (d0, m) = d.div_mod(&ten);
                sum += digit_sum(m.to_uint());
                d = d0;
            }
            // following core causes core dumps...
            // let s = n.to_str();
            // for str::each_char(s) |c| {
            //     let d = char::to_digit(c, 10);
            //     sum += d.get();
            // }
            max = uint::max(max, sum);
        }
    }

    return max.to_str();
}
