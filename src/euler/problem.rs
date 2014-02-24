use std::io;
use time;
use term::Terminal;
use term::color::Color;
use term::color;

static WARN_COLOR_NSEC:  u64 = 1  * NSEC_PER_SEC;
static ERROR_COLOR_NSEC: u64 = 10 * NSEC_PER_SEC;
static NSEC_PER_SEC:     u64 = 1000000000;

static COLOR_OK:   Color = color::GREEN;
static COLOR_NG:   Color = color::RED;
static COLOR_WARN: Color = color::YELLOW;

pub struct Problem<'a> {
    id: uint,
    answer: &'a str,
    solve: fn() -> ~str
}

impl<'a> Problem<'a> {
    pub fn solve(&self) -> Solution {
        let (time, answer) = bench(|| (self.solve)());
        Solution {
            name: self.id.to_str(),
            is_correct: answer.equiv(&self.answer),
            answer: answer,
            time: time
        }
    }
}

pub struct Solution {
    priv name: ~str,
    priv is_correct: bool,
    priv answer: ~str,
    priv time: u64
}

impl Solution {
    pub fn new(name: ~str, is_correct: bool, answer: ~str, time: u64) -> Solution {
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
        fn ok(s: ~str) -> (Option<Color>, ~str) { (Some(COLOR_OK), s) }
        fn ng(s: ~str) -> (Option<Color>, ~str) { (Some(COLOR_NG), s) }
        fn warn(s: ~str) -> (Option<Color>, ~str) { (Some(COLOR_WARN), s) }
        fn normal(s: ~str) -> (Option<Color>, ~str) { (None, s) }

        let result_mark = if self.is_correct { ok(~"OK") } else { ng(~"NG") };
        let time_str = format!(" {:13}", nanosec_to_str(self.time));
        let time_mark = if !enable_time_color || self.time < WARN_COLOR_NSEC {
            normal(time_str)
        } else if self.time < ERROR_COLOR_NSEC {
            warn(time_str)
        } else {
            ng(time_str)
        };

        print_items(&[
                normal(~"["), result_mark, normal(~"]"),
                normal(format!(" {:5}", self.name)),
                time_mark,
                normal(format!(" {:20}", self.answer)),
                normal(~"\n")
                ]);
    }
}

fn nanosec_to_str(nsec: u64) -> ~str {
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

fn print_items(items: &[(Option<Color>, ~str)]) {
    match Terminal::new(io::stdout()) {
        Err(_) => {
            let mut out = io::stdout();
            for &(_, ref s) in items.iter() { let _ = out.write_str(*s); }
        },
        Ok(mut t) => {
            for &(c, ref s) in items.iter() {
                match c {
                    Some(c) => { let _ = t.fg(c); let _ = t.write_str(*s); let _ = t.reset(); }
                    None    => { let _ = t.write_str(*s); }
                }
            }
        }
    }
}
