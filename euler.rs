extern mod std;

#[path="./common/mod.rs"]
mod common;

#[path="./prob0001_0050/mod.rs"]
mod prob0001_0050;

priv use common::problem::{ Problem };

fn solve_all(problems: &[Problem]) {
    for problems.each |p| { p.solve(); }
}

fn main() {
    solve_all(prob0001_0050::problems);
}
