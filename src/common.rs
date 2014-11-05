#![warn(unused, bad_style,
        unused_qualifications, unused_typecasts, unused_results)]

#![feature(macro_rules, slicing_syntax)]

extern crate curl;
extern crate time;
extern crate serialize;

use std::error::{Error, FromError};
use std::{fmt, os};
use std::io::{mod, IoResult, File};
use std::io::fs::{mod, PathExtensions};
use curl::http;
use serialize::json;

enum SolverError {
    Io(io::IoError),
    Http(curl::ErrCode)
}

impl FromError<io::IoError> for SolverError {
    fn from_error(err: io::IoError) -> SolverError { Io(err) }
}
impl FromError<curl::ErrCode> for SolverError {
    fn from_error(err: curl::ErrCode) -> SolverError { Http(err) }
}

impl Error for SolverError {
    fn description(&self) -> &str {
        match *self {
            Io(ref err) => err.description(),
            Http(ref err) => err.description()
        }
    }

    fn detail(&self) -> Option<String> {
        match *self {
            Io(ref err) => err.detail(),
            Http(ref err) => err.detail()
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            Io(ref err) => Some(err as &Error),
            Http(ref err) => Some(err as &Error)
        }
    }
}

impl fmt::Show for SolverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Io(ref err) => write!(f, "{}", err),
            Http(ref err) => write!(f, "{}", err)
        }
    }
}

#[deriving(Show, Encodable, Decodable)]
pub struct SolverResult<T> {
    pub time: u64,
    pub answer: T,
    pub is_ok: bool
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
        Solver { answer: answer, solver: FnOnly(solver) }
    }

    pub fn new_with_file(
        answer: &'a str, file_name: &'a str, solver: fn(File) -> IoResult<String>
    ) -> Solver<'a> {
        Solver { answer: answer, solver: FnWithFile(file_name, solver) }
    }

    pub fn run(self) {
        match self.solve() {
            Err(err) => {
                let _ = writeln!(&mut io::stderr(), "{}: {}", os::args()[0], err);
                os::set_exit_status(255);
            }
            Ok(result) => {
                if !result.is_ok {
                    os::set_exit_status(1);
                }
                io::stdio::println(json::encode(&result)[]);
            }
        }
    }

    fn solve(&self) -> Result<SolverResult<String>, SolverError> {
        let (time, answer) = match self.solver {
            FnOnly(fun) => bench(proc() fun()),
            FnWithFile(file_name, fun) => {
                let file = try!(setup_file(file_name));
                let (time, answer) = bench(proc() fun(file));
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

fn bench<T>(f: proc() -> T) -> (u64, T) {
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
