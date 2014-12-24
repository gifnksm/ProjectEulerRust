#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

extern crate common;

use std::cmp;
use std::io::{BufferedReader, File, IoResult};
use common::Solver;

fn read_matrix<T: Reader>(reader: T) -> IoResult<Vec<Vec<uint>>> {
    let mut br = BufferedReader::new(reader);

    let mut mat = vec![];

    for line in br.lines() {
        let row = try!(line).trim().split(',').filter_map(StrExt::parse::<uint>).collect();
        mat.push(row);
    }

    Ok(mat)
}

fn minimal_path_sum(mat: Vec<Vec<uint>>) -> uint {
    let (w, h) = (mat[0].len(), mat.len());

    let mut sum = Vec::from_fn(h, |_y| Vec::from_elem(w, 0u));

    for y in range(0, h) { sum[y][0] = mat[y][0]; }
    for x in range(1, w) {
        for y in range(0, h) {
            let mut min = sum[y][x - 1];

            let mut s = 0;
            for dy in range(1, y) {
                s += mat[y - dy][x];
                min = cmp::min(sum[y - dy][x - 1] + s, min);
            }

            let mut s = 0;
            for dy in range(1, h - y) {
                s += mat[y + dy][x];
                min = cmp::min(sum[y + dy][x - 1] + s, min);
            }

            sum[y][x] = mat[y][x] + min;
        }
    }

    range(0, h)
        .map(|y| sum[y][w - 1])
        .min()
        .unwrap()
}

fn solve(file: File) -> IoResult<String> {
    let mat = try!(read_matrix(file));
    Ok(minimal_path_sum(mat).to_string())
}

fn main() { Solver::new_with_file("260324", "p082_matrix.txt", solve).run(); }

#[cfg(test)]
mod tests {
    #[test]
    fn five_by_five() {
        let mat = vec![
            vec![131, 673, 234, 103,  18],
            vec![201,  96, 342, 965, 150],
            vec![630, 803, 746, 422, 111],
            vec![537, 699, 497, 121, 956],
            vec![805, 732, 524,  37, 331]
        ];
        assert_eq!(994, super::minimal_path_sum(mat));
    }
}
