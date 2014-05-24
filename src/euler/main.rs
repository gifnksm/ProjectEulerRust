#![crate_id = "euler"]
#![crate_type = "bin"]

extern crate time;
extern crate common;
extern crate term;

use std::os;
use std::iter;
use problem::{Problem, Solution};

mod problem;
mod problem_list;

trait ProblemIterator {
    fn solve_all(&mut self) -> bool;
}

impl<T: Iterator<&'static Problem<'static>>> ProblemIterator for T {
    fn solve_all(&mut self) -> bool {
        let mut total_time  = 0;
        let mut solve_cnt   = 0;
        let mut all_correct = true;

        for p in &mut *self {
            let sol = p.solve();
            sol.print(true);

            total_time += sol.time();
            solve_cnt  += 1;
            all_correct &= sol.is_correct();
        }

        if solve_cnt > 1 {
            let avg_time = total_time / solve_cnt;
            Solution::new("AVG".to_owned(),   all_correct, "".to_owned(), avg_time).print(true);
            Solution::new("TOTAL".to_owned(), all_correct, "".to_owned(), total_time).print(false);
        }

        all_correct
    }
}

fn parse_range(s: &str) -> Option<(uint, uint)> {
    if !s.contains_char('-') {
        from_str::<uint>(s).map(|n| (n, n))
    } else {
        let mut ss = s.splitn('-', 1);
        let opt_a = ss.next().and_then(|sa| from_str(sa));
        let opt_b = ss.next().and_then(|sb| from_str(sb));
        opt_a.and_then(|a| opt_b.map(|b| (a, b)))
    }
}

fn main() {
    let args = os::args();
    let args = args.tail();

    let all_correct = if args.is_empty() {
        problem_list::PROBLEMS.iter()
            .map(|x| *x)
            .solve_all()
    } else {
        args.iter()
            .filter_map(|s| parse_range(s.as_slice()))
            .flat_map(|(a, b)| iter::range_inclusive(a, b))
            .filter_map(|id| problem_list::PROBLEMS.bsearch(|p| p.id.cmp(&id)))
            .map(|i| problem_list::PROBLEMS[i])
            .solve_all()
    };

    if !all_correct { os::set_exit_status(1) }
}
