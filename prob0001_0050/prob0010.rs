use common::prime::{ Prime };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 10,
    answer: "142913828922",
    solver: solve
};

fn solve() -> ~str {
    let mut sum = 0;
    let mut ps = Prime();
    for ps.each |p| {
        if p >= 2000000 {
            break;
        }
        sum += p;
    }
    return sum.to_str();
}
