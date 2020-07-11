#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use attohttpc::StatusCode;
use failure::Fail;
use getopts::Options;
use num_integer::Integer;
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    env, fmt,
    fs::{self, File},
    io,
    io::prelude::*,
    path::PathBuf,
    process,
    time::Instant,
};
use term::{color, color::Color};

type OutputPair<'a> = (Option<Color>, Cow<'a, str>);

const NSEC_PER_SEC: u128 = 1000000000;
const NSEC_WARN_LIMIT: u128 = NSEC_PER_SEC;
const NSEC_NG_LIMIT: u128 = 10 * NSEC_PER_SEC;

const COLOR_OK: Color = color::GREEN;
const COLOR_NG: Color = color::RED;
const COLOR_WARN: Color = color::YELLOW;

#[derive(Fail, Debug, Clone)]
#[fail(display = "{}, {}", status, body)]
struct InvalidHttpStatusError {
    status: StatusCode,
    body: String,
}

pub type Result<T> = std::result::Result<T, failure::Error>;

#[derive(Debug, Serialize, Deserialize)]
pub struct SolverResult<T> {
    pub time: u128,
    pub answer: T,
    pub is_ok: bool,
}

impl<T: Serialize> SolverResult<T> {
    pub fn print_json<W: Write>(&self, out: &mut W) -> Result<()> {
        let _ = writeln!(out, "{}", serde_json::to_string(self)?)?;
        Ok(())
    }
}

fn print_items(items: &[OutputPair<'_>]) {
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
        items.push(normal(format!("{} ", name)));

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
            answer,
            solver: SolverFn::FnOnly(solver),
        }
    }

    pub fn new_with_file(
        answer: &'a str,
        file_name: &'a str,
        solver: fn(File) -> io::Result<String>,
    ) -> Solver<'a> {
        Solver {
            answer,
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

    pub fn solve(&self) -> Result<SolverResult<String>> {
        let (time, answer) = match self.solver {
            SolverFn::FnOnly(fun) => bench(fun),
            SolverFn::FnWithFile(file_name, fun) => {
                let file = setup_file(file_name)?;
                let (time, answer) = bench(move || fun(file));
                (time, answer?)
            }
        };

        let result = SolverResult {
            is_ok: answer == self.answer,
            time,
            answer,
        };
        Ok(result)
    }
}

fn bench<T, F: FnOnce() -> T>(f: F) -> (u128, T) {
    let start_time = Instant::now();
    let result = f();
    let nsec = start_time.elapsed().as_nanos();
    (nsec, result)
}

fn setup_file(file_name: &str) -> Result<File> {
    let mut path = PathBuf::from("./.cache");
    path.push(file_name);
    if !path.is_file() {
        fs::create_dir_all(&path.parent().unwrap())?;
        let content = download(file_name)?;
        File::create(&path)?.write_all(&content)?;
    }

    let file = File::open(&path)?;
    Ok(file)
}

const BASE_URL: &str = "https://projecteuler.net/project/resources/";
fn download(file_name: &str) -> Result<Vec<u8>> {
    let url = format!("{}{}", BASE_URL, file_name);

    for retry in 0.. {
        let resp = attohttpc::get(&url).send()?;
        let status = resp.status();
        let body = resp.bytes()?;
        if status.is_success() {
            return Ok(body);
        }

        let err = InvalidHttpStatusError {
            status,
            body: String::from_utf8_lossy(&body).into(),
        };
        let program = env::args().next().unwrap();
        let _ = writeln!(&mut io::stderr(), "{}: {}", program, err);
        if retry >= 3 {
            return Err(err.into());
        }
    }
    unreachable!();
}

pub fn init() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();
}

#[macro_export]
macro_rules! problem {
    ($answer:expr, $solver:expr) => {
        fn main() {
            $crate::init();
            $crate::Solver::new($answer, $solver).run();
        }

        #[test]
        fn test_solve() {
            assert!($crate::Solver::new($answer, $solver).solve().unwrap().is_ok);
        }
    };
    ($answer:expr, $file:expr, $solver:expr) => {
        fn main() {
            $crate::init();
            $crate::Solver::new_with_file($answer, $file, $solver).run();
        }

        #[test]
        fn test_solve() {
            assert!(
                $crate::Solver::new_with_file($answer, $file, $solver)
                    .solve()
                    .unwrap()
                    .is_ok
            );
        }
    };
}
