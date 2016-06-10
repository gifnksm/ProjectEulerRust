#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results)]

extern crate curl;
extern crate getopts;
extern crate num;
extern crate rustc_serialize;
extern crate term;
extern crate time;

use std::borrow::Cow;
use std::error::Error;
use std::{env, fmt, io, process};
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::PathBuf;
use curl::http;
use getopts::Options;
use num::Integer;
use rustc_serialize::{Encodable, json};
use term::color;
use term::color::Color;

type OutputPair<'a> = (Option<Color>, Cow<'a, str>);

const NSEC_PER_SEC: u64 = 1000000000;
const NSEC_WARN_LIMIT: u64 = 1 * NSEC_PER_SEC;
const NSEC_NG_LIMIT: u64 = 10 * NSEC_PER_SEC;

const COLOR_OK: Color = color::GREEN;
const COLOR_NG: Color = color::RED;
const COLOR_WARN: Color = color::YELLOW;

#[derive(Debug)]
pub enum SolverError {
    Io(io::Error),
    Http(curl::ErrCode),
}

impl From<io::Error> for SolverError {
    fn from(err: io::Error) -> SolverError {
        SolverError::Io(err)
    }
}
impl From<curl::ErrCode> for SolverError {
    fn from(err: curl::ErrCode) -> SolverError {
        SolverError::Http(err)
    }
}

impl Error for SolverError {
    fn description(&self) -> &str {
        match *self {
            SolverError::Io(ref err) => err.description(),
            SolverError::Http(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            SolverError::Io(ref err) => Some(err),
            SolverError::Http(ref err) => Some(err),
        }
    }
}

impl fmt::Display for SolverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SolverError::Io(ref err) => write!(f, "{}", err),
            SolverError::Http(ref err) => write!(f, "{}", err),
        }
    }
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct SolverResult<T> {
    pub time: u64,
    pub answer: T,
    pub is_ok: bool,
}

impl<T: Encodable> SolverResult<T> {
    pub fn print_json<W: Write>(&self, out: &mut W) -> io::Result<()> {
        writeln!(out, "{}", json::encode(self).unwrap())
    }
}

fn print_items(items: &[OutputPair]) {
    match term::stdout() {
        None => {
            let mut out = io::stdout();
            for &(_, ref s) in items {
                let _ = write!(&mut out, "{}", s);
            }
            let _ = out.flush();
        }
        Some(mut t) => {
            for &(c, ref s) in items {
                match c {
                    Some(c) => {
                        let _ = t.fg(c);
                        let _ = write!(&mut t, "{}", s);
                        let _ = t.reset();
                    }
                    None => {
                        let _ = write!(&mut t, "{}", s);
                    }
                }
            }
            let _ = t.flush();
        }
    }
}

impl<T: fmt::Display> SolverResult<T> {
    pub fn print_pretty(&self, name: &str, enable_time_color: bool) -> io::Result<()> {
        let mut items = vec![];
        if self.is_ok {
            items.push(normal(format!("{} ", name)));
        } else {
            items.push(normal(format!("{} ", name)));
        }

        items.push(normal("["));
        if self.is_ok {
            items.push(ok("OK"));
        } else {
            items.push(ng("NG"));
        }
        items.push(normal("] "));

        let (sec, nsec) = self.time.div_rem(&NSEC_PER_SEC);
        let time_str = format!("{:3}.{:09} ", sec, nsec);
        if !enable_time_color || self.time < NSEC_WARN_LIMIT {
            items.push(normal(time_str));
        } else if self.time < NSEC_NG_LIMIT {
            items.push(warn(time_str));
        } else {
            items.push(ng(time_str));
        }

        items.push(normal(format!("{} ", self.answer)));

        items.push(normal("\n"));
        print_items(&items);

        fn normal<'a, T: Into<Cow<'a, str>>>(s: T) -> OutputPair<'a> {
            (None, s.into())
        }
        fn ok<'a, T: Into<Cow<'a, str>>>(s: T) -> OutputPair<'a> {
            (Some(COLOR_OK), s.into())
        }
        fn warn<'a, T: Into<Cow<'a, str>>>(s: T) -> OutputPair<'a> {
            (Some(COLOR_WARN), s.into())
        }
        fn ng<'a, T: Into<Cow<'a, str>>>(s: T) -> OutputPair<'a> {
            (Some(COLOR_NG), s.into())
        }

        Ok(())
    }
}

enum SolverFn<'a> {
    FnOnly(fn() -> String),
    FnWithFile(&'a str, fn(File) -> io::Result<String>),
}

pub struct Solver<'a> {
    answer: &'a str,
    solver: SolverFn<'a>,
}

impl<'a> Solver<'a> {
    pub fn new(answer: &'a str, solver: fn() -> String) -> Solver<'a> {
        Solver {
            answer: answer,
            solver: SolverFn::FnOnly(solver),
        }
    }

    pub fn new_with_file(answer: &'a str,
                         file_name: &'a str,
                         solver: fn(File) -> io::Result<String>)
                         -> Solver<'a> {
        Solver {
            answer: answer,
            solver: SolverFn::FnWithFile(file_name, solver),
        }
    }

    pub fn run(self) {
        let args = env::args().collect::<Vec<_>>();
        let program = &args[0];

        let mut opts = Options::new();
        let _ = opts.optflag("", "json", "Output JSON format");
        let _ = opts.optflag("h", "help", "Display this message");

        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(f) => {
                let _ = writeln!(&mut io::stderr(), "{}: {}", program, f);
                process::exit(255);
            }
        };

        if matches.opt_present("h") {
            let short = opts.short_usage(&program);
            println!("{}", opts.usage(&short));
            return;
        }

        match self.solve() {
            Err(err) => {
                let _ = writeln!(&mut io::stderr(), "{}: {}", program, err);
                process::exit(255);
            }
            Ok(result) => {
                if matches.opt_present("json") {
                    let _ = result.print_json(&mut io::stdout());
                } else {
                    let _ = result.print_pretty(&program, true);
                }
                if !result.is_ok {
                    process::exit(1);
                }
            }
        }
    }

    pub fn solve(&self) -> Result<SolverResult<String>, SolverError> {
        let (time, answer) = match self.solver {
            SolverFn::FnOnly(fun) => bench(move || fun()),
            SolverFn::FnWithFile(file_name, fun) => {
                let file = try!(setup_file(file_name));
                let (time, answer) = bench(move || fun(file));
                (time, try!(answer))
            }
        };

        let result = SolverResult {
            is_ok: answer == self.answer,
            time: time,
            answer: answer,
        };
        Ok(result)
    }
}

fn bench<T, F: FnOnce() -> T>(f: F) -> (u64, T) {
    let start_time = time::precise_time_ns();
    let result = f();
    let end_time = time::precise_time_ns();
    let nsec = end_time - start_time;
    (nsec, result)
}

fn setup_file(file_name: &str) -> Result<File, SolverError> {
    let mut path = PathBuf::from("./.cache");
    path.push(file_name);
    if !path.is_file() {
        try!(fs::create_dir_all(&path.parent().unwrap()));
        let mut file = try!(File::create(&path));
        let content = try!(download(file_name));
        try!(file.write_all(&content));
    }

    let file = try!(File::open(&path));
    Ok(file)
}

const BASE_URL: &'static str = "https://projecteuler.net/project/resources/";
fn download(file_name: &str) -> Result<Vec<u8>, curl::ErrCode> {
    let url = format!("{}{}", BASE_URL, file_name);

    let mut retry = 0;
    loop {
        let result = http::handle().get(&url[..]).exec();
        match result {
            Ok(resp) => {
                return Ok(resp.move_body());
            }
            Err(err) => {
                let program = env::args().next().unwrap();
                let _ = writeln!(&mut io::stderr(), "{}: {}", program, err);
                retry += 1;
                if retry >= 3 {
                    return Err(err);
                }
            }
        }
    }
}

#[macro_export]
macro_rules! problem {
    ($answer:expr, $solver:expr) => (
        #[cfg(not(test))]
        fn main() {
            $crate::Solver::new($answer, $solver).run();
        }

        #[test]
        fn test_solve() {
            assert!($crate::Solver::new($answer, $solver).solve().unwrap().is_ok);
        }
    );
    ($answer:expr, $file:expr, $solver:expr) => (
        #[cfg(not(test))]
        fn main() {
            $crate::Solver::new_with_file($answer, $file, $solver).run();
        }

        #[test]
        fn test_solve() {
            assert!($crate::Solver::new_with_file($answer, $file, $solver).solve().unwrap().is_ok);
        }
     );
}
