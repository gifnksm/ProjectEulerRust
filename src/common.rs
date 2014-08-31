#![warn(unused, bad_style,
        unnecessary_qualification, unnecessary_typecast, unused_result)]

extern crate time;
extern crate serialize;

use std::{io, os};
use serialize::json;

#[deriving(Show, Encodable, Decodable)]
pub struct SolveResult<T> {
    pub time: u64,
    pub answer: T,
    pub is_ok: bool
}

pub struct Solver {
    answer: &'static str,
    solve: fn () -> String
}

impl Solver {
    pub fn new(answer: &'static str, solve: fn() -> String) -> Solver {
        Solver { answer: answer, solve: solve }
    }

    pub fn run(&self) {
        let (time, answer) = bench(|| (self.solve)());
        let is_ok = answer.as_slice() == self.answer;
        let result = SolveResult {
            time: time,
            answer: answer,
            is_ok: is_ok
        };
        io::stdio::println(json::encode(&result).as_slice());

        if !is_ok {
            os::set_exit_status(1);
        }
    }
}

fn bench<T>(f: || -> T) -> (u64, T) {
    let start_time = time::precise_time_ns();
    let result     = f();
    let end_time   = time::precise_time_ns();
    let nsec       = end_time - start_time;
    (nsec, result)
}
