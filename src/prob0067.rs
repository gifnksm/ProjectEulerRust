#[crate_id = "prob0067"];
#[crate_type = "rlib"];

use std::{cmp, vec};
use std::io::{BufferedReader, File};

pub static EXPECTED_ANSWER: &'static str = "7273";

pub fn solve() -> ~str {
    let mut br = BufferedReader::new(
        File::open(&Path::new("files/triangle.txt")).expect("file not found."));

    let triangle = br.lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.words().filter_map(from_str::<uint>).to_owned_vec())
        .to_owned_vec();
    let init = triangle.init();
    let last = triangle.last().unwrap();
    init.rev_iter().fold(last.to_owned(), |prev, elem| {
            vec::from_fn(elem.len(), |i| {
                    elem[i] + cmp::max(prev[i], prev[i + 1])
                })
        })[0].to_str()
}

