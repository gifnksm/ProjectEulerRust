#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(old_orphan_check, macro_rules, slicing_syntax)]

extern crate curl;
extern crate getopts;
extern crate num;
extern crate "rustc-serialize" as rustc_serialize;
extern crate term;
extern crate time;

use std::borrow::IntoCow;
use std::error::{Error, FromError};
use std::{fmt, os};
use std::io::{mod, IoResult, File};
use std::io::fs::{mod, PathExtensions};
use std::str::CowString;
use curl::http;
use num::Integer;
use rustc_serialize::{json, Encodable};
use term::{color, Terminal};
use term::color::Color;

type OutputPair<'a> = (Option<Color>, CowString<'a>);

const NSEC_PER_SEC:    u64 = 1000000000;
const NSEC_WARN_LIMIT: u64 = 1  * NSEC_PER_SEC;
const NSEC_NG_LIMIT:   u64 = 10 * NSEC_PER_SEC;

const COLOR_OK:   Color = color::GREEN;
const COLOR_NG:   Color = color::RED;
const COLOR_WARN: Color = color::YELLOW;

pub enum SolverError {
    Io(io::IoError),
    Http(curl::ErrCode)
}

impl FromError<io::IoError> for SolverError {
    fn from_error(err: io::IoError) -> SolverError { SolverError::Io(err) }
}
impl FromError<curl::ErrCode> for SolverError {
    fn from_error(err: curl::ErrCode) -> SolverError { SolverError::Http(err) }
}

impl Error for SolverError {
    fn description(&self) -> &str {
        match *self {
            SolverError::Io(ref err) => err.description(),
            SolverError::Http(ref err) => err.description()
        }
    }

    fn detail(&self) -> Option<String> {
        match *self {
            SolverError::Io(ref err) => err.detail(),
            SolverError::Http(ref err) => err.detail()
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            SolverError::Io(ref err) => Some(err as &Error),
            SolverError::Http(ref err) => Some(err as &Error)
        }
    }
}

impl fmt::Show for SolverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SolverError::Io(ref err) => write!(f, "{}", err),
            SolverError::Http(ref err) => write!(f, "{}", err)
        }
    }
}

#[deriving(Show, RustcEncodable, RustcDecodable)]
pub struct SolverResult<T> {
    pub time: u64,
    pub answer: T,
    pub is_ok: bool
}

impl<T: for<'a> Encodable<json::Encoder<'a>, fmt::Error>> SolverResult<T> {
    pub fn print_json<W: Writer>(&self, out: &mut W) -> IoResult<()> {
        out.write_line(json::encode(self)[])
    }
}

fn print_items(items: &[OutputPair]) {
    match term::stdout() {
        None => {
            let mut out = io::stdout();
            for &(_, ref s) in items.iter() {
                let _ = out.write_str(s.as_slice());
            }
            let _ = out.flush();
        }
        Some(mut t) => {
            for &(c, ref s) in items.iter() {
                match c {
                    Some(c) => {
                        let _ = t.fg(c);
                        let _ = t.write_str(s.as_slice());
                        let _ = t.reset();
                    }
                    None => {
                        let _ = t.write_str(s.as_slice());
                    }
                }
            }
            let _ = t.flush();
        }
    }
}

impl<T: fmt::Show> SolverResult<T> {
    pub fn print_pretty(&self, name: &str, enable_time_color: bool) -> IoResult<()> {
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
        print_items(items[]);

        fn normal<'a, T: IntoCow<'a, String, str>>(s: T) -> OutputPair<'a> {
            (None, s.into_cow())
        }
        fn ok<'a, T: IntoCow<'a, String, str>>(s: T) -> OutputPair<'a> {
            (Some(COLOR_OK), s.into_cow())
        }
        fn warn<'a, T: IntoCow<'a, String, str>>(s: T) -> OutputPair<'a> {
            (Some(COLOR_WARN), s.into_cow())
        }
        fn ng<'a, T: IntoCow<'a, String, str>>(s: T) -> OutputPair<'a> {
            (Some(COLOR_NG), s.into_cow())
        }

        Ok(())
    }
}

enum SolverFn<'a> {
    FnOnly(fn() -> String),
    FnWithFile(&'a str, fn(File) -> IoResult<String>)
}

pub struct Solver<'a> {
    answer: &'a str,
    solver: SolverFn<'a>
}

impl<'a> Solver<'a> {
    pub fn new(answer: &'a str, solver: fn() -> String) -> Solver<'a> {
        Solver { answer: answer, solver: SolverFn::FnOnly(solver) }
    }

    pub fn new_with_file(
        answer: &'a str, file_name: &'a str, solver: fn(File) -> IoResult<String>
    ) -> Solver<'a> {
        Solver { answer: answer, solver: SolverFn::FnWithFile(file_name, solver) }
    }

    pub fn run(self) {
        let args = os::args();
        let program = &args[0];

        let opts = &[
            getopts::optflag("", "json", "Output JSON format"),
            getopts::optflag("h", "help", "Display this message")
        ];

        let matches = match getopts::getopts(args[1 ..], opts) {
            Ok(m) => m,
            Err(f) => {
                let _ = writeln!(&mut io::stderr(), "{}: {}", program, f);
                os::set_exit_status(255);
                return
            }
        };

        if matches.opt_present("h") {
            let short = getopts::short_usage(program[], opts);
            println!("{}", getopts::usage(short[], opts));
            return
        }

        match self.solve() {
            Err(err) => {
                let _ = writeln!(&mut io::stderr(), "{}: {}", program, err);
                os::set_exit_status(255);
            }
            Ok(result) => {
                if !result.is_ok {
                    os::set_exit_status(1);
                }
                if matches.opt_present("json") {
                    let _ = result.print_json(&mut io::stdout());
                } else {
                    let _ = result.print_pretty(program[], true);
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
            is_ok:  answer[] == self.answer,
            time:   time,
            answer: answer
        };
        Ok(result)
    }
}

fn bench<T, F: FnOnce() -> T>(f: F) -> (u64, T) {
    let start_time = time::precise_time_ns();
    let result     = f();
    let end_time   = time::precise_time_ns();
    let nsec       = end_time - start_time;
    (nsec, result)
}

fn setup_file(file_name: &str) -> Result<File, SolverError> {
    let path = Path::new(format!(".cache/{}", file_name));
    if !path.is_file() {
        let dir_path = path.dir_path();
        try!(fs::mkdir_recursive(&dir_path, io::USER_RWX));
        let mut file = try!(File::create(&path));
        let content = try!(download(file_name));
        try!(file.write(content[]));
    }

    let file = try!(File::open(&path));
    Ok(file)
}

const BASE_URL: &'static str = "https://projecteuler.net/project/resources/";
fn download(file_name: &str) -> Result<Vec<u8>, curl::ErrCode> {
    let url = format!("{}{}", BASE_URL, file_name);
    let resp = try!(http::handle()
                    .get(url)
                    .exec());
    Ok(resp.move_body())
}

#[macro_escape]

#[macro_export]
macro_rules! problem {
    ($answer:expr, $solver:expr) => (
        #[cfg(not(test))]
        fn main() {
            ::common::Solver::new($answer, $solver).run();
        }

        #[test]
        fn test_solve() {
            assert!(::common::Solver::new($answer, $solver).solve().unwrap().is_ok);
        }
    );
    ($answer:expr, $file:expr, $solver:expr) => (
        #[cfg(not(test))]
        fn main() {
            ::common::Solver::new_with_file($answer, $file, $solver).run();
        }

        #[test]
        fn test_solve() {
            assert!(::common::Solver::new_with_file($answer, $file, $solver).solve().unwrap().is_ok);
        }
     );
}
