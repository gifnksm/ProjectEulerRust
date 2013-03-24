extern mod std;

#[path="./common/mod.rs"]
mod common;

#[path="./prob0001_0050/mod.rs"]
mod prob0001_0050;
#[path="./prob0051_0100/mod.rs"]
mod prob0051_0100;

priv use std::time::{ precise_time_ns };

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

priv static NSEC_PER_SEC: u64 = 1000000000;
priv fn nanosec_to_str(nsec: u64) -> ~str {
    return fmt!("%3u.%09u",
         (nsec / NSEC_PER_SEC) as uint,
         (nsec % NSEC_PER_SEC) as uint);
}

priv fn solve(p: &Problem) -> u64 {
    let start_time = precise_time_ns();
    let comp_answer = (p.solver)();
    let calc_time   = precise_time_ns() - start_time;

    assert_eq!(comp_answer, p.answer.to_str());

    io::println(fmt!("%-5u %-12s %s",
                     p.id, p.answer, nanosec_to_str(calc_time)));

    return calc_time;
}

fn main() {
    let nums = os::args().filter_mapped(|&s| uint::from_str(s));

    let mut total_time = 0;
    let mut solve_cnt = 0;
    if nums.is_empty() {
        for each_problems |p| {
            total_time += solve(p);
            solve_cnt += 1;
        }
    } else {
        for nums.each_val |n| {
            for each_problems |p| {
                if p.id == n {
                    total_time += solve(p);
                    solve_cnt += 1;
                }
            }
        }
    }

    if solve_cnt > 1 {
        io::println(fmt!("TOTAL %-12s %s", "", nanosec_to_str(total_time)));
    }
}
