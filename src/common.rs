#![warn(unused, bad_style,
        unnecessary_qualification, unnecessary_typecast, unused_result)]

#![feature(macro_rules, slicing_syntax)]

extern crate curl;
extern crate time;
extern crate serialize;

use std::os;
use std::io::{mod, IoResult, File};
use std::io::fs::{mod, PathExtensions};
use curl::http;
use serialize::json;

macro_rules! try2(
    ($e:expr) => (try!($e.map_err(|e| e.to_solver_error())))
)

#[deriving(Show)]
enum SolverErrorKind {
    HttpError(curl::ErrCode),
    IoError(io::IoErrorKind)
}

#[deriving(Show)]
struct SolverError {
    kind: SolverErrorKind,
    desc: Option<&'static str>,
    detail: Option<String>
}

trait ToSolverError {
    fn to_solver_error(self) -> SolverError;
}

impl ToSolverError for curl::ErrCode {
    fn to_solver_error(self) -> SolverError {
        SolverError { kind: HttpError(self), desc: None, detail: None }
    }
}

impl ToSolverError for io::IoError {
    fn to_solver_error(self) -> SolverError {
        SolverError {
            kind: IoError(self.kind),
            desc: Some(self.desc),
            detail: self.detail
        }
    }
}

#[deriving(Show, Encodable, Decodable)]
pub struct SolveResult<T> {
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
        let (time, answer) = match self.solver {
            FnOnly(fun) => bench(proc() fun()),
            FnWithFile(file_name, fun) => {
                let file = match setup_file(file_name) {
                    Ok(file) => file,
                    Err(e) => {
                        let _ = writeln!(&mut io::stderr(), "{}", e);
                        os::set_exit_status(255);
                        return
                    }
                };
                match bench(proc() fun(file)) {
                    (_, Err(e)) => {
                        let _ = writeln!(&mut io::stderr(), "{}", e);
                        os::set_exit_status(255);
                        return
                    }
                    (time, Ok(anser)) => (time, anser)
                }
            }
        };

        let is_ok = answer[] == self.answer;
        let result = SolveResult {
            time:   time,
            answer: answer,
            is_ok:  is_ok
        };
        io::stdio::println(json::encode(&result)[]);

        if !is_ok {
            os::set_exit_status(1);
        }
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
        try2!(fs::mkdir_recursive(&dir_path, io::USER_RWX));
        let mut file = try2!(File::create(&path));
        let content = try2!(download(file_name));
        try2!(file.write(content[]));
    }

    let file = try2!(File::open(&path));
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
