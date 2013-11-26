#[feature(managed_boxes)];

extern mod extra;
extern mod common;

use std::{io, os};
use std::iter::Range;
use extra::{time, term};
use extra::term::Terminal;
use problem::Problem;

mod problem;

static NSEC_PER_SEC: u64 = 1000000000;
fn nanosec_to_str(nsec: u64) -> ~str {
    return format!("{}.{:09}",
         (nsec / NSEC_PER_SEC) as uint,
         (nsec % NSEC_PER_SEC) as uint);
}

fn bench<T>(f: &fn() -> T) -> (u64, T) {
    let start_time = time::precise_time_ns();
    let result     = f();
    let end_time   = time::precise_time_ns();
    return (end_time - start_time, result);
}

fn color_print<T: Writer>(writer: T, color: term::color::Color, s: &str) {
    let mut term = Terminal::new(writer);
    match term { Ok(ref mut t) => { t.fg(color); }, _ => {}}
    print(s);
    match term { Ok(ref mut t) => { t.reset(); }, _ => {}}
}

fn print_result(correct: bool, name: &str, time: u64, comp_answer: &str) {
    print("[");
    if correct {
        color_print(io::stdout(), term::color::GREEN, "OK");
    } else {
        color_print(io::stdout(), term::color::RED, "NG");
    }
    println!("] {:5} {:13} {:20}", name, nanosec_to_str(time), comp_answer);
}

struct ArgIterator<'self> {
    args: &'self [~str],
    idx: uint,
    cur_range: Range<uint>,
}

impl<'self> ArgIterator<'self> {
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
        println!("{:?}", ns);
    }
}

impl<'self> Iterator<uint> for ArgIterator<'self> {
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
        print_result(correct, p.id.to_str(), time, answer);

        total_time += time;
        solve_cnt  += 1;
        all_correct &= correct;
    }

    if solve_cnt > 1 {
        print_result(all_correct, "TOTAL", total_time, "");
    }
}

fn main() {
    let args = os::args();
    let args = args.tail();

    if args.is_empty() {
        solve_all(range(0, problem::PROBLEMS.len())
                  .map(|i| problem::PROBLEMS[i]))
    } else {
        solve_all(ArgIterator::new(args)
                  .filter_map(|n| problem::PROBLEMS.bsearch(|&p| p.id.cmp(&n)))
                  .map(|i| problem::PROBLEMS[i]));
    };
}
