extern mod extra;
extern mod common;

use std::iterator::{ Iterator, IteratorUtil };
use extra::time;
use extra::term;
use common::problem::{ Problem };
use common::extiter::{ Range };

mod problem;

static NSEC_PER_SEC: u64 = 1000000000;
fn nanosec_to_str(nsec: u64) -> ~str {
    return fmt!("%u.%09u",
         (nsec / NSEC_PER_SEC) as uint,
         (nsec % NSEC_PER_SEC) as uint);
}

fn bench<T>(f: &fn() -> T) -> (u64, T) {
    let start_time = time::precise_time_ns();
    let result     = f();
    let end_time   = time::precise_time_ns();
    return (end_time - start_time, result);
}

fn color_print(writer: @io::Writer, color: u8, s: &str) {
    term::fg(writer, color);
    print(s);
    term::reset(writer);
}

fn print_result(correct: bool, name: &str, time: u64, comp_answer: &str) {
    print("[");
    if correct {
        color_print(io::stdout(), term::color_green, "OK");
    } else {
        color_print(io::stdout(), term::color_red, "NG");
    }
    println(fmt!("] %5s %13s %20s", name, nanosec_to_str(time), comp_answer));
}

fn parse_num(s: &str) -> ~[uint] {
    if !s.contains_char('-') {
        return old_iter::to_vec(&uint::from_str(s));
    }
    let mut ns = ~[];
    for s.each_split_char('-') |ss| {
        match uint::from_str(ss) {
            Some(n) => { ns.push(n); }
            None    => { return ~[]; }
        }
    }
    if ns.len() > 2 { return ~[]; }
    return Range::new(ns[0], ns[1] + 1).to_vec();
}

struct ArgIterator<'self> {
    args: &'self [~str],
    idx: uint,
    cur_range: Range<uint>,
}

impl<'self> ArgIterator<'self> {
    pub fn new<'a>(args: &'a [~str]) -> ArgIterator<'a> {
        let mut it = ArgIterator { args: args, idx: 0, cur_range: Range::new(0u, 0) };
        it.update_range();
        return it;
    }

    priv fn update_range(&mut self) {
        self.cur_range = Range::new(0u, 0);
        if self.idx >= self.args.len() { return; }

        if !self.args[self.idx].contains_char('-') {
            for uint::from_str(self.args[self.idx]).each |&n| {
                self.cur_range = Range::new(n, n + 1);
            }
            return;
        }

        let mut ns = ~[];
        for self.args[self.idx].each_split_char('-') |ss| {
            match uint::from_str(ss) {
                Some(n) => { ns.push(n); }
                None    => { return; }
            }
        }
        if ns.len() > 2 { return; }
        self.cur_range = Range::new(ns[0], ns[1] + 1);
        println(fmt!("%?", ns));
    }
}

impl<'self> Iterator<uint> for ArgIterator<'self> {
    pub fn next(&mut self) -> Option<uint> {
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

fn main() {
    let args = os::args();
    let mut it = if args.tail().is_empty() {
        @Range::new(0, problem::problems.len())
            .transform(|i| problem::problems[i])
            as @Iterator<&Problem>
    } else {
        @ArgIterator::new(args.tail()).filter_map(|n| {
            vec::bsearch(problem::problems, |&p| p.id.cmp(&n))
                .map(|&i| problem::problems[i])
        }) as @Iterator<&Problem>
    };


    let mut total_time  = 0;
    let mut solve_cnt   = 0;
    let mut all_correct = true;

    loop {
        match it.next() {
            Some(&p) => {
                let (time, answer) = do bench { (p.solver)() };
                let correct = p.answer == answer;
                print_result(correct, p.id.to_str(), time, answer);

                total_time += time;
                solve_cnt  += 1;
                all_correct &= correct;
            }
            None => { break; }
        }
    }

    if solve_cnt > 1 {
        print_result(all_correct, "TOTAL", total_time, "");
    }
}
