#[link(name = "prob0091", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 91,
    answer: "14234",
    solver: solve
};

fn count_right_o(x_max: uint, y_max: uint) -> uint {
    return x_max * y_max;
}

fn count_right_p(x_max: uint, y_max: uint) -> uint {
    let mut cnt = x_max * y_max; // (0, y0) - (xi, y0) => xi: [1, x_max], y0: [0, y_max]

    for uint::range(1, x_max + 1) |x| {
        for uint::range(1, y_max + 1) |y| {
            let d = x.gcd(&y);
            let (dx, neg_dy) = (y / d, x / d);
            cnt += uint::min(y / neg_dy, (x_max - x) / dx);
        }
    }

    return cnt;
}


pub fn solve() -> ~str {
    let (x_max, y_max) = (50, 50);
    let answer = count_right_o(x_max, y_max) + count_right_p(x_max, y_max) * 2;
    return answer.to_str();
}
