#![warn(bad_style, unused, unused_extern_crates, unused_import_braces, unused_qualifications,
        unused_results)]

extern crate common;
#[macro_use]
extern crate error_chain;
extern crate glob;
extern crate serde_json;

use common::SolverResult;
use glob::Paths;
use std::env;
use std::io;
use std::io::prelude::*;
use std::os::unix::process::ExitStatusExt;
use std::path::Path;
use std::process;
use std::process::Command;
use std::str;

const PROBLEM_EXE_PAT: &'static str = "p[0-9][0-9][0-9]";

error_chain! {
    foreign_links {
        Io(io::Error);
        Json(serde_json::Error);
        Glob(glob::GlobError);
        GlobPattern(glob::PatternError);
    }
}

fn problem_paths(dir_path: &Path) -> Result<Paths> {
    let pat = dir_path.join(PROBLEM_EXE_PAT);
    match pat.to_str() {
        Some(x) => Ok(glob::glob(x)?),
        None => Err(Error::from("path contains non-utf8 character")),
    }
}

fn run_problem(path: &Path) -> Result<SolverResult<String>> {
    let proc_out = Command::new(path).arg("--json").output()?;

    if !proc_out.stderr.is_empty() {
        let _ = match str::from_utf8(&proc_out.stderr) {
            Ok(s) => writeln!(&mut io::stderr(), "{}", s.trim()),
            Err(e) => writeln!(&mut io::stderr(), "{:?}: {}", proc_out.stderr, e),
        };
    }

    match proc_out.status.code() {
        Some(0) | Some(1) => {} // expected
        Some(st) => {
            return Err(Error::from(format!("child process exit with {}", st)));
        }
        None => {
            return Err(Error::from(format!(
                "child process exit with siglan {}",
                proc_out.status.signal().unwrap()
            )));
        }
    }

    let result = serde_json::from_reader(&mut &proc_out.stdout[..])?;
    Ok(result)
}

fn run() -> Result<bool> {
    let dir_path = {
        let mut path = env::current_exe()?;
        let _ = path.pop();
        path
    };
    let mut out = io::stdout();

    let mut is_ok = true;
    let mut num_prob = 0;
    let mut total_time = 0;
    for path in problem_paths(&dir_path)? {
        let path = path?;
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
