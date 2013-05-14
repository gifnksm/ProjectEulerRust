#[link(name = "prob0082", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 82,
    answer: "260324",
    solver: solve
};

pub fn solve() -> ~str {
    let result = io::file_reader(&Path("files/matrix.txt")).map(|file| {
        let mut mat = ~[];
        for file.each_line |line| {
            let mut row = ~[];
            for line.each_split_char(',') |n| {
                row.push(uint::from_str(n).get());
            }
            mat.push(row);
            assert_eq!(mat[0].len(), mat.last().len());
        }
        let w = mat[0].len();
        let h = mat.len();
        ((w, h), mat)
    }).map(|&((w, h), mat)| {
        let mut sum = vec::from_fn(h, |_y| vec::from_elem(w, 0));
        for uint::range(0, h) |y| { sum[y][0] = mat[y][0]; }
        for uint::range(1, w) |x| {
            for uint::range(0, h) |y| {
                let mut min = sum[y][x - 1];

                let mut s = 0;
                for uint::range(1, y) |dy| {
                    s += mat[y - dy][x];
                    min = uint::min(sum[y - dy][x - 1] + s, min);
                }

                let mut s = 0;
                for uint::range(1, h - y) |dy| {
                    s += mat[y + dy][x];
                    min = uint::min(sum[y + dy][x - 1] + s, min);
                }

                sum[y][x] = mat[y][x] + min;
            }
        }
        let mut min = uint::max_value;
        for uint::range(0, h) |y| {
            min = uint::min(sum[y][w - 1], min);
        }
        min
    });

    match result {
        Err(msg) => fail!(msg),
        Ok(value) => return value.to_str()
    }
}
