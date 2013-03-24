use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 1,
    answer: "233168",
    solver: solve
};

fn solve() -> ~str {
    let mut sum = 0;
    for uint::range(0, 1000) |n| {
        if n % 3 == 0 || n % 5 == 0 {
            sum += n;
        }
    }
    return sum.to_str();
}
