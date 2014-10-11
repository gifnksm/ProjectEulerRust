use std::io;
use time;
use term;
use term::Terminal;
use term::color::Color;
use term::color;

const WARN_COLOR_NSEC:  u64 = 1  * NSEC_PER_SEC;
const ERROR_COLOR_NSEC: u64 = 10 * NSEC_PER_SEC;
const NSEC_PER_SEC:     u64 = 1000000000;

const COLOR_OK:   Color = color::GREEN;
const COLOR_NG:   Color = color::RED;
const COLOR_WARN: Color = color::YELLOW;

pub struct Problem<'a> {
    pub id: uint,
    pub answer: &'a str,
    pub solve: fn() -> String
}

impl<'a> Problem<'a> {
    pub fn solve(&self) -> Solution {
        let (time, answer) = bench(|| (self.solve)());
        Solution {
            name: self.id.to_string(),
            is_correct: answer.equiv(&self.answer),
            answer: answer,
            time: time
        }
    }
}

pub struct Solution {
    name: String,
    is_correct: bool,
    answer: String,
    time: u64
}

impl Solution {
    pub fn new(name: String, is_correct: bool, answer: String, time: u64) -> Solution {
        Solution {
            name: name,
            is_correct: is_correct,
            answer: answer,
            time: time
        }
    }

    #[inline] pub fn is_correct(&self) -> bool { self.is_correct }
    #[inline] pub fn time(&self) -> u64 { self.time }

    pub fn print(&self, enable_time_color: bool) {
        fn ok<'a>(s: &'a str) -> (Option<Color>, &'a str) { (Some(COLOR_OK), s) }
        fn ng<'a>(s: &'a str) -> (Option<Color>, &'a str) { (Some(COLOR_NG), s) }
        fn warn<'a>(s: &'a str) -> (Option<Color>, &'a str) { (Some(COLOR_WARN), s) }
        fn normal<'a>(s: &'a str) -> (Option<Color>, &'a str) { (None, s) }

        let result_mark = if self.is_correct { ok("OK") } else { ng("NG") };
        let name_str = format!(" {:5}", self.name);
        let time_str = format!(" {:13}", nanosec_to_str(self.time));
        let time_mark = if !enable_time_color || self.time < WARN_COLOR_NSEC {
            normal(time_str.as_slice())
        } else if self.time < ERROR_COLOR_NSEC {
            warn(time_str.as_slice())
        } else {
            ng(time_str.as_slice())
        };
        let answer_str = format!(" {:20}", self.answer);

        print_items(&[
                normal("["), result_mark, normal("]"),
                normal(name_str.as_slice()),
                time_mark,
                normal(answer_str.as_slice()),
                normal("\n")
                ]);
    }
}

fn nanosec_to_str(nsec: u64) -> String {
    return format!("{}.{:09}",
         (nsec / NSEC_PER_SEC) as uint,
         (nsec % NSEC_PER_SEC) as uint);
}

fn bench<T>(f: || -> T) -> (u64, T) {
    let start_time = time::precise_time_ns();
    let result     = f();
    let end_time   = time::precise_time_ns();
    return (end_time - start_time, result);
}

fn print_items(items: &[(Option<Color>, &str)]) {
    match term::stdout() {
        None => {
            let mut out = io::stdout();
            for &(_, s) in items.iter() { let _ = out.write_str(s); }
        },
        Some(mut t) => {
            for &(c, s) in items.iter() {
                match c {
                    Some(c) => { let _ = t.fg(c); let _ = t.write_str(s); let _ = t.reset(); }
                    None    => { let _ = t.write_str(s); }
                }
            }
        }
    }
}
