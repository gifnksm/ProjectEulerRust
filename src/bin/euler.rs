#![warn(unused, bad_style,
        unnecessary_qualification, unnecessary_typecast, unused_result)]

#![feature(macro_rules)]

extern crate glob;
extern crate num;
extern crate serialize;
extern crate term;
extern crate common;

use std::{io, os, str};
use std::io::{Command, MemReader};
use std::io::process::ExitStatus;
use std::path::Display;
use std::str::{MaybeOwned, SendStr};
use glob::Paths;
use num::Integer;
use serialize::{json, Decodable};
use term::{color, Terminal};
use term::color::Color;
use common::SolveResult;

static PROBLEM_EXE_PAT: &'static str = "p[0-9][0-9][0-9]";

static NSEC_PER_SEC:    u64 = 1000000000;
static NSEC_WARN_LIMIT: u64 = 1  * NSEC_PER_SEC;
static NSEC_NG_LIMIT:   u64 = 10 * NSEC_PER_SEC;

static COLOR_OK:   Color = color::GREEN;
static COLOR_NG:   Color = color::RED;
static COLOR_WARN: Color = color::YELLOW;

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
    fn to_program_error(self: Self) -> ProgramError;
}

impl ToProgramError for io::IoError {
    fn to_program_error(self: io::IoError) -> ProgramError {
        ProgramError::new(self.desc.into_maybe_owned(), IoError(self))
    }
}

impl ToProgramError for json::ParserError {
    fn to_program_error(self: json::ParserError) -> ProgramError {
        match self {
            json::SyntaxError(code, line, col) => {
                ProgramError::new(format!("{}:{}:{}", line, col, json::error_str(code)),
                                  JsonSyntaxError(code, line, col))
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
        ProgramError::new(format!("{}", self), JsonDecoderError(self))
    }
}

fn exe_path() -> ProgramResult<Path> {
    match os::self_exe_name() {
        Some(x) => Ok(x),
        None    => Err(ProgramError::new("cannot get self exe name", Unknown))
    }
}

fn problem_paths(dir_path: Path) -> ProgramResult<Paths> {
    let pat = dir_path.join(PROBLEM_EXE_PAT);
    match pat.as_str() {
        Some(x) => Ok(glob::glob(x)),
        None    => Err(ProgramError::new("path contains non-utf8 character", Unknown))
    }
}

fn run_problem(path: &Path) -> ProgramResult<SolveResult<String>> {
    let proc_out = try2!(Command::new(path).output());

    if !proc_out.error.is_empty() {
        let _ = match str::from_utf8(proc_out.error.as_slice()) {
            Some(s) => writeln!(&mut io::stderr(), "{}", s.trim()),
            None    => writeln!(&mut io::stderr(), "{}", proc_out.error)
        };
    }

    match proc_out.status {
        ExitStatus(0) | ExitStatus(1) => { } // expected
        st => {
            return Err(ProgramError::new(format!("child process exit with {}", st), Unknown))
        }
    }

    let json = try2!(json::from_reader(&mut MemReader::new(proc_out.output)));
    Ok(try2!(Decodable::decode(&mut json::Decoder::new(json))))
}

fn print_items<'a>(items: &[OutputPair]) {
    match term::stdout() {
        None => {
            let mut out = io::stdout();
            for &(_, ref s) in items.iter() { let _ = out.write_str(s.as_slice()); }
            let _ = out.flush();
        },
        Some(mut t) => {
            for &(c, ref s) in items.iter() {
                match c {
                    Some(c) => { let _ = t.fg(c); let _ = t.write_str(s.as_slice()); let _ = t.reset(); }
                    None    => { let _ = t.write_str(s.as_slice()); }
                }
            }
            let _ = t.flush();
        }
    }
}

fn print_result<'a, T: GenericPath>(name: Display<T>, result: ProgramResult<SolveResult<String>>) {
    let mut items = vec![];
    match result {
        Ok(r) => {
            items.push(normal("["));
            if r.is_ok {
                items.push(ok("OK"));
            } else {
                items.push(ng("NG"));
            }
            items.push(normal("] "));
            items.push(normal(format!("{} ", name)));
            items.push(normal(format!("{:20} ", r.answer)));
            let (sec, nsec) = r.time.div_rem(&NSEC_PER_SEC);
            let time_str = format!("{:3}.{:09}", sec, nsec);
            if r.time < NSEC_WARN_LIMIT {
                items.push(normal(time_str));
            } else if r.time < NSEC_NG_LIMIT {
                items.push(warn(time_str));
            } else {
                items.push(ng(time_str));
            }
        }
        Err(e) => {
            items.push(normal("["));
            items.push(ng("!!"));
            items.push(normal("] "));
            items.push(normal(format!("{} ", name)));
            items.push(normal(format!("{}", e)));
            os::set_exit_status(1);
        }
    }
    items.push(normal("\n"));
    print_items(items.as_slice());

    fn normal<'a, T: IntoMaybeOwned<'a>>(s: T) -> OutputPair<'a> {
        (None, s.into_maybe_owned())
    }
    fn ok<'a, T: IntoMaybeOwned<'a>>(s: T) -> OutputPair<'a> {
        (Some(COLOR_OK), s.into_maybe_owned())
    }
    fn warn<'a, T: IntoMaybeOwned<'a>>(s: T) -> OutputPair<'a> {
        (Some(COLOR_WARN), s.into_maybe_owned())
    }
    fn ng<'a, T: IntoMaybeOwned<'a>>(s: T) -> OutputPair<'a> {
        (Some(COLOR_NG), s.into_maybe_owned())
    }
}

fn run() -> ProgramResult<()> {
    let dir_path = try!(exe_path()).dir_path();

    for path in try!(problem_paths(dir_path)) {
        print_result(path.filename_display(), run_problem(&path));
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
