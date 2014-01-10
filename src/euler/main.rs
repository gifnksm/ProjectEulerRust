#[crate_id = "euler"];
#[crate_type = "bin"];

#[feature(managed_boxes)];

extern mod extra;
extern mod common;

use std::{io, os};
use std::iter::Range;
use extra::{time, term};
use extra::term::Terminal;
use problem::Problem;

mod problem;
mod problem_list;

static NSEC_PER_SEC: u64 = 1000000000;
fn nanosec_to_str(nsec: u64) -> ~str {
    return format!("{}.{:09}",
         (nsec / NSEC_PER_SEC) as uint,
         (nsec % NSEC_PER_SEC) as uint);
}

fn bench<T>(f: proc() -> T) -> (u64, T) {
    let start_time = time::precise_time_ns();
    let result     = f();
    let end_time   = time::precise_time_ns();
    return (end_time - start_time, result);
}

fn color_print(color: term::color::Color, s: &str) {
    match Terminal::new(io::stdout()) {
        Ok(ref mut term) => {
            term.fg(color);
            term.write(s.as_bytes());
            term.reset();
        }
        Err(_) => { io::stdout().write(s.as_bytes()) }
    }
}

fn print_result(name: &str, time: u64, comp_answer: &str, time_color: bool, correct: bool) {
    let out = &mut io::stdout();
    write!(out, "[");
    if correct {
        color_print(term::color::GREEN, "OK");
    } else {
        color_print(term::color::RED, "NG");
    }
    write!(out, "] {:5}", name);
    if (time_color && time > 10 * NSEC_PER_SEC) {
        color_print(term::color::RED, format!(" {:13}", nanosec_to_str(time)));
    } else if time_color && time > NSEC_PER_SEC {
        color_print(term::color::YELLOW, format!(" {:13}", nanosec_to_str(time)));
    } else {
        write!(out, " {:13}", nanosec_to_str(time));
    }
    writeln!(out, " {:20}", comp_answer);
}

struct ArgIterator<'a> {
    args: &'a [~str],
    idx: uint,
    cur_range: Range<uint>,
}

impl<'a> ArgIterator<'a> {
    pub fn new<'a>(args: &'a [~str]) -> ArgIterator<'a> {
        let mut it = ArgIterator { args: args, idx: 0, cur_range: range(0u, 0) };
        it.update_range();
        return it;
    }

    fn update_range(&mut self) {
        self.cur_range = range(0u, 0);
        if self.idx >= self.args.len() { return; }

        if !self.args[self.idx].contains_char('-') {
            let n = from_str::<uint>(self.args[self.idx]);
            for &n in n.iter() {
                self.cur_range = range(n, n + 1);
            }
            return;
        }

        let mut ns = ~[];
        for ss in self.args[self.idx].split('-') {
            match from_str::<uint>(ss) {
                Some(n) => { ns.push(n); }
                None    => { return; }
            }
        }
        if ns.len() > 2 { return; }
        self.cur_range = range(ns[0], ns[1] + 1);
    }
}

impl<'a> Iterator<uint> for ArgIterator<'a> {
    fn next(&mut self) -> Option<uint> {
        loop {
            if self.idx >= self.args.len() { return None; }
            match self.cur_range.next() {
                Some(n) => { return Some(n); }
                None    => {
                    self.idx += 1;
                    self.update_range();
                }
            }
        }
    }
}

fn solve_all<T: Iterator<&'static Problem<'static>>>(mut it: T) {
    let mut total_time  = 0;
    let mut solve_cnt   = 0;
    let mut all_correct = true;

    for p in it {
        let (time, answer) = do bench { (p.solve)() };
        let correct = p.answer == answer;
        print_result(p.id.to_str(), time, answer, true, correct);

        total_time += time;
        solve_cnt  += 1;
        all_correct &= correct;
    }

    if solve_cnt > 1 {
        print_result("AVG", total_time / solve_cnt, "", true, all_correct);
        print_result("TOTAL", total_time, "", false, all_correct);
    }

    if !all_correct {
        os::set_exit_status(1);
    }
}

fn main() {
    let args = os::args();
    let args = args.tail();

    if args.is_empty() {
        solve_all(range(0, problem_list::PROBLEMS.len())
                  .map(|i| problem_list::PROBLEMS[i]))
    } else {
        solve_all(ArgIterator::new(args)
                  .filter_map(|n| problem_list::PROBLEMS.bsearch(|&p| p.id.cmp(&n)))
                  .map(|i| problem_list::PROBLEMS[i]));
    };
}
