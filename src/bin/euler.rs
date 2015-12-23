#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results)]

extern crate glob;
extern crate rustc_serialize as rustc_serialize;
extern crate term;
extern crate common;

use std::borrow::Cow;
use std::env;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;
use std::str;
use std::os::unix::process::ExitStatusExt;
use std::process;
use glob::Paths;
use rustc_serialize::Decodable;
use rustc_serialize::json::{self, Json};
use term::color::Color;
use common::SolverResult;

const PROBLEM_EXE_PAT: &'static str = "p[0-9][0-9][0-9]";

type ProgramResult<T> = Result<T, ProgramError>;
type OutputPair<'a> = (Option<Color>, Cow<'a, str>);

#[derive(Debug)]
enum ProgramError {
    IoError(io::Error),
    JsonParserError(json::ParserError),
    JsonDecoderError(json::DecoderError),
    Unknown(Cow<'static, str>),
}

impl ProgramError {
    fn unknown<T: Into<Cow<'static, str>>>(msg: T) -> ProgramError {
        ProgramError::Unknown(msg.into())
    }
}

impl From<io::Error> for ProgramError {
    fn from(err: io::Error) -> ProgramError {
        ProgramError::IoError(err)
    }
}

impl From<json::ParserError> for ProgramError {
    fn from(err: json::ParserError) -> ProgramError {
        ProgramError::JsonParserError(err)
    }
}

impl From<json::DecoderError> for ProgramError {
    fn from(err: json::DecoderError) -> ProgramError {
        ProgramError::JsonDecoderError(err)
    }
}

fn problem_paths(dir_path: &Path) -> ProgramResult<Paths> {
    let pat = dir_path.join(PROBLEM_EXE_PAT);
    match pat.to_str() {
        Some(x) => Ok(glob::glob(x).unwrap()),
        None => Err(ProgramError::unknown("path contains non-utf8 character")),
    }
}

fn run_problem(path: &Path) -> ProgramResult<SolverResult<String>> {
    let proc_out = try!(Command::new(path).arg("--json").output());

    if !proc_out.stderr.is_empty() {
        let _ = match str::from_utf8(&proc_out.stderr) {
            Ok(s) => writeln!(&mut io::stderr(), "{}", s.trim()),
            Err(e) => writeln!(&mut io::stderr(), "{:?}: {}", proc_out.stderr, e),
        };
    }

    match proc_out.status.code() {
        Some(0) | Some(1) => {} // expected
        Some(st) => {
            return Err(ProgramError::unknown(format!("child process exit with {}", st)));
        }
        None => {
            return Err(ProgramError::unknown(format!("child process exit with siglan {}",
                                                     proc_out.status.signal().unwrap())));
        }
    }

    let json = try!(Json::from_reader(&mut &proc_out.stdout[..]));
    Ok(try!(Decodable::decode(&mut json::Decoder::new(json))))
}

fn run() -> ProgramResult<bool> {
    let dir_path = {
        let mut path = try!(env::current_exe());
        path.pop();
        path
    };
    let mut out = io::stdout();

    let mut is_ok = true;
    let mut num_prob = 0;
    let mut total_time = 0;
    for path in try!(problem_paths(&dir_path)) {
        let path = path.unwrap();
        let program = path.file_name().unwrap().to_string_lossy().to_string();

        match run_problem(&path) {
            Ok(ref r) => {
                num_prob += 1;
                total_time += r.time;
                is_ok &= r.is_ok;
                let _ = r.print_pretty(&program, true);
            }
            Err(e) => {
                is_ok = false;
                let _ = writeln!(&mut out, "{}: {:?}", program, e);
            }
        }
    }

    if num_prob > 0 {
        let r = SolverResult {
            time: total_time / num_prob,
            answer: "".to_string(),
            is_ok: is_ok,
        };
        let _ = r.print_pretty(" AVG", true);

        let r = SolverResult {
            time: total_time,
            answer: "".to_string(),
            is_ok: is_ok,
        };
        let _ = r.print_pretty(" SUM", false);
    }

    Ok(is_ok)
}

fn main() {
    match run() {
        Ok(true) => process::exit(0),
        Ok(false) => process::exit(1),
        Err(e) => {
            let _ = writeln!(&mut io::stderr(), "{:?}", e);
            process::exit(255);
        }
    }
}
