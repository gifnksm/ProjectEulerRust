use common::calc::{ each_fib };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 2,
    answer: "4613732",
    solver: solve
};

fn solve() -> ~str {
    let max = 4000000;
    let mut sum = 0;
    for each_fib |f: &uint| {
        if *f >= max { break; }
        if *f % 2 == 0 { sum += *f; }
    }
    return sum.to_str();
}
