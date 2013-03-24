extern mod std;

#[path="./common/mod.rs"]
mod common;

#[path="./prob0001_0050/mod.rs"]
mod prob0001_0050;

priv use common::problem::{ Problem };

priv fn each_problems(f: &fn(&Problem) -> bool) {
    for prob0001_0050::problems.each |p| {
        if !f(p) { return; }
    }
}

priv fn solve_all() {
    for each_problems |p| { p.solve(); }
}

priv fn solve(n: uint) {
    for each_problems |p| {
        if p.number == n { p.solve(); }
    }
}

fn main() {
    let nums = os::args().filter_mapped(|&s| uint::from_str(s));
    if nums.is_empty() {
        solve_all();
    } else {
        for nums.each_val |n| {
            solve(n);
        }
    }
}
