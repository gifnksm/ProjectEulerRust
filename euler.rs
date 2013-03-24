extern mod std;

#[path="./common/mod.rs"]
mod common;

#[path="./prob0001_0050/mod.rs"]
mod prob0001_0050;
#[path="./prob0051_0100/mod.rs"]
mod prob0051_0100;

priv use common::problem::{ Problem };

priv static problem_sets: &'static [&'static [&'static Problem<'static>]] = &[
    prob0001_0050::problems,
    prob0051_0100::problems
];

priv fn each_problems(f: &fn(&Problem) -> bool) {
    for problem_sets.each_val |ps| {
        for ps.each_val |p| {
            if !f(p) { return; }
        }
    }
}

priv fn solve(p: &Problem) {
    io::println(fmt!("Problem #%u: %s", p.id, p.answer));
    assert_eq!((p.solver)(), p.answer.to_str());
}

fn main() {
    let nums = os::args().filter_mapped(|&s| uint::from_str(s));
    if nums.is_empty() {
        for each_problems |p| { solve(p); }
    } else {
        for nums.each_val |n| {
            for each_problems |p| {
                if p.id == n { solve(p); }
            }
        }
    }
}
