extern mod std;

#[path="./common/mod.rs"]
mod common;

#[path="./prob0001_0050/mod.rs"]
mod prob0001_0050;

priv use common::problem::{ Problem };

priv fn each_problems(f: &fn(&Problem) -> bool) {
    for prob0001_0050::problems.each_val |p| {
        if !f(p) { return; }
    }
}

priv fn solve(p: &Problem) {
    assert_eq!((p.solver)(), p.answer.to_str());
    io::println(fmt!("Problem #%u: %s", p.id, p.answer));
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
