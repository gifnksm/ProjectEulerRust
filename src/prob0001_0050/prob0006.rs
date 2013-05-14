use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 6,
    answer: "25164150",
    solver: solve
};

fn sum_of_square(n: uint) -> uint { n * (n + 1) * (2 * n + 1) / 6 }
fn sum_of_seq(n: uint) -> uint { n * (n + 1) / 2 }
fn square_of_sum(n: uint) -> uint {
    let s = sum_of_seq(n);
    return s * s;
}

fn solve() -> ~str {
    let sq_of_sum = square_of_sum(100);
    let sum_of_sq = sum_of_square(100);
    return (sq_of_sum - sum_of_sq).to_str();
}
