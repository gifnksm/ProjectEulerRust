#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(if_let, macro_rules, slicing_syntax)]

extern crate glob;
extern crate serialize;
extern crate term;
extern crate common;

use std::{io, os, str};
use std::io::{Command, MemReader};
use std::io::process::ExitStatus;
use std::str::{MaybeOwned, SendStr};
use glob::Paths;
use serialize::{json, Decodable};
use term::color::Color;
use common::SolverResult;

const PROBLEM_EXE_PAT: &'static str = "p[0-9][0-9][0-9]";

macro_rules! try2(
    ($e:expr) => (try!($e.map_err(|e| e.to_program_error())))
)

type ProgramResult<T> = Result<T, ProgramError>;
type OutputPair<'a> = (Option<Color>, MaybeOwned<'a>);

#[deriving(Show)]
enum ProgramErrorKind {
    IoError(io::IoError),
    JsonSyntaxError(json::ErrorCode, uint, uint),
    JsonDecoderError(json::DecoderError),
    Unknown
}

#[deriving(Show)]
struct ProgramError {
    kind: ProgramErrorKind,
    message: SendStr
}

impl ProgramError {
    pub fn new<T: IntoMaybeOwned<'static>>(msg: T, kind: ProgramErrorKind) -> ProgramError {
        ProgramError {
            kind: kind,
            message: msg.into_maybe_owned()
        }
    }
}

trait ToProgramError {
    fn to_program_error(self) -> ProgramError;
}

impl ToProgramError for io::IoError {
    fn to_program_error(self) -> ProgramError {
        ProgramError::new(self.desc.into_maybe_owned(), ProgramErrorKind::IoError(self))
    }
}

impl ToProgramError for json::ParserError {
    fn to_program_error(self) -> ProgramError {
        match self {
            json::SyntaxError(code, line, col) => {
                ProgramError::new(format!("{}:{}:{}", line, col, json::error_str(code)),
                                  ProgramErrorKind::JsonSyntaxError(code, line, col))
            },
            json::IoError(kind, desc) => {
                (io::IoError {kind: kind, desc: desc, detail: None })
                    .to_program_error()
            }
        }
    }
}

impl ToProgramError for json::DecoderError {
    fn to_program_error(self: json::DecoderError) -> ProgramError {
        ProgramError::new(format!("{}", self), ProgramErrorKind::JsonDecoderError(self))
    }
}

fn exe_path() -> ProgramResult<Path> {
    match os::self_exe_name() {
        Some(x) => Ok(x),
        None    => Err(ProgramError::new("cannot get self exe name", ProgramErrorKind::Unknown))
    }
}

fn problem_paths(dir_path: Path) -> ProgramResult<Paths> {
    let pat = dir_path.join(PROBLEM_EXE_PAT);
    match pat.as_str() {
        Some(x) => Ok(glob::glob(x)),
        None    => Err(ProgramError::new("path contains non-utf8 character", ProgramErrorKind::Unknown))
    }
}

fn run_problem(path: &Path) -> ProgramResult<SolverResult<String>> {
    let proc_out = try2!(Command::new(path).arg("--json").output());

    if !proc_out.error.is_empty() {
        let _ = match str::from_utf8(proc_out.error[]) {
            Some(s) => writeln!(&mut io::stderr(), "{}", s.trim()),
            None    => writeln!(&mut io::stderr(), "{}", proc_out.error)
        };
    }

    match proc_out.status {
        ExitStatus(0) | ExitStatus(1) => { } // expected
        st => {
            return Err(ProgramError::new(format!("child process exit with {}", st), ProgramErrorKind::Unknown))
        }
    }

    let json = try2!(json::from_reader(&mut MemReader::new(proc_out.output)));
    Ok(try2!(Decodable::decode(&mut json::Decoder::new(json))))
}

fn run() -> ProgramResult<()> {
    let dir_path = try!(exe_path()).dir_path();
    let mut out = io::stdout();

    let mut is_ok = true;
    let mut num_prob = 0;
    let mut total_time = 0;
    for path in try!(problem_paths(dir_path)) {
        let program = format!("{}", path.filename_display());

        match run_problem(&path) {
            Ok(ref r) => {
                num_prob   += 1;
                total_time += r.time;
                is_ok &= r.is_ok;
                let _ = r.print_pretty(program[], true);
            }
            Err(e) => {
                is_ok = false;
                let _ = writeln!(out, "{}: {}", program, e);
            }
        }
    }

    if num_prob > 0 {
        let r = SolverResult {
            time: total_time / num_prob,
            answer: "".to_string(),
            is_ok: is_ok
        };
        let _ = r.print_pretty(" AVG", true);

        let r = SolverResult {
            time: total_time,
            answer: "".to_string(),
            is_ok: is_ok
        };
        let _ = r.print_pretty(" SUM", false);
    }

    if !is_ok {
        os::set_exit_status(1);
    }

    Ok(())
}

fn main() {
    match run() {
        Ok(()) => {}
        Err(e) => {
            let _ = writeln!(&mut io::stderr(), "{}", e);
            os::set_exit_status(255);
        }
    }
}
