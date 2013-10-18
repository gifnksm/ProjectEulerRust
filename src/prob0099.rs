#[link(name = "prob0099", vers = "0.0")];
#[crate_type = "lib"];

use std::io;

pub static EXPECTED_ANSWER: &'static str = "709";

pub fn solve() -> ~str {
    let result = io::file_reader(&Path::new("files/base_exp.txt"))
        .map(|input| {
            let mut line_idx = 1u;
            let mut max = 0.0;
            let mut max_idx = 1;
            do input.each_line |line| {
                let opt = line.find(',');
                for &idx in opt.iter() {
                    let base: f64 = from_str(line.slice(0, idx)).unwrap();
                    let exp:  f64 = from_str(line.slice(idx + 1, line.len())).unwrap();
                    let ln = exp * base.ln();
                    if ln > max {
                        max = ln;
                        max_idx = line_idx;
                    }
                    line_idx += 1;
                }
                true
            };
            max_idx
        });

    match result {
        Err(msg) => fail!(msg),
        Ok(value) => value.to_str()
    }
}
