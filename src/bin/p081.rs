//! [Problem 81](https://projecteuler.net/problem=81) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#[macro_use(problem)] extern crate common;

use std::cmp;
use std::old_io::{BufferedReader, File, IoResult};

fn read_matrix<T: Reader>(reader: T) -> IoResult<Vec<Vec<u32>>> {
    let mut br = BufferedReader::new(reader);

    let mut mat = vec![];

    for line in br.lines() {
        let row = try!(line).trim().split(',').filter_map(StrExt::parse::<u32>).collect();
        mat.push(row);
    }

    Ok(mat)
}

fn minimal_path_sum(mut mat: Vec<Vec<u32>>) -> u32 {
    let (w, h) = (mat[0].len(), mat.len());

    for y in (1 .. h) {
        mat[y][0] += mat[y - 1][0];
    }
    for x in (1 .. w) {
        mat[0][x] += mat[0][x - 1];
        for y in (1 .. h) {
            mat[y][x] += cmp::min(mat[y - 1][x], mat[y][x - 1]);
        }
    }
    mat[h - 1][w - 1]
}

fn solve(file: File) -> IoResult<String> {
    let mat = try!(read_matrix(file));
    Ok(minimal_path_sum(mat).to_string())
}

problem!("427337", "p081_matrix.txt", solve);

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
        assert_eq!(2427, super::minimal_path_sum(mat));
    }
}
