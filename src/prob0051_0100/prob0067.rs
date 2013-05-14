#[link(name = "prob0067", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 67,
    answer: "7273",
    solver: solve
};

pub fn solve() -> ~str {
    let result = io::file_reader(&Path("files/triangle.txt")).map(|file| {
        let mut triangle = ~[];
        for file.each_line |line| {
            let mut nums = ~[];
            for str::each_word(line) |word| {
                nums.push(uint::from_str(word).get())
            }
            triangle.push(nums);
        }
        triangle
    }).map(|triangle| {
        let init = triangle.init();
        let last = triangle.last();
        (do init.foldr(last.to_owned()) |elm, prev| {
            do vec::from_fn(elm.len()) |i| {
                elm[i] + uint::max(prev[i], prev[i + 1])
            }
        })[0]
    });

    match result {
        Err(msg) => fail!(msg),
        Ok(value) => return value.to_str()
    }
}

