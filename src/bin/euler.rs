#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use common::SolverResult;
use failure::bail;
use glob::Paths;
use std::{
    env, io, io::prelude::*, os::unix::process::ExitStatusExt, path::Path, process,
    process::Command, str,
};

const PROBLEM_EXE_PAT: &str = "p[0-9][0-9][0-9]";

type Result<T> = std::result::Result<T, failure::Error>;

fn problem_paths(dir_path: &Path) -> Result<Paths> {
    let pat = dir_path.join(PROBLEM_EXE_PAT);
    match pat.to_str() {
        Some(x) => Ok(glob::glob(x)?),
        None => bail!("path contains non-utf8 character"),
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
        Some(st) => bail!("child process exit with {}", st),
        None => bail!(
            "child process exit with signal {}",
            proc_out.status.signal().unwrap()
        ),
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
            is_ok,
        };
        let _ = r.print_pretty(" AVG", true);

        let r = SolverResult {
            time: total_time,
            answer: "".to_string(),
            is_ok,
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
