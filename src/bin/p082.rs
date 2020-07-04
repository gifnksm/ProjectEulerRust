//! [Problem 82](https://projecteuler.net/problem=82) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use std::{
    cmp,
    fs::File,
    io::{self, prelude::*, BufReader},
};

fn read_matrix<T: Read>(reader: T) -> io::Result<Vec<Vec<u32>>> {
    let mut mat = vec![];

    for line in BufReader::new(reader).lines() {
        let row = line?
            .trim()
            .split(',')
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();
        mat.push(row);
    }

    Ok(mat)
}

fn minimal_path_sum(mat: Vec<Vec<u32>>) -> u32 {
    let (w, h) = (mat[0].len(), mat.len());

    let mut sum = vec![vec![0; w]; h];

    for y in 0..h {
        sum[y][0] = mat[y][0];
    }
    for x in 1..w {
        for y in 0..h {
            let mut min = sum[y][x - 1];

            let mut s = 0;
            for dy in 1..y {
                s += mat[y - dy][x];
                min = cmp::min(sum[y - dy][x - 1] + s, min);
            }

            let mut s = 0;
            for dy in 1..(h - y) {
                s += mat[y + dy][x];
                min = cmp::min(sum[y + dy][x - 1] + s, min);
            }

            sum[y][x] = mat[y][x] + min;
        }
    }

    (0..h).map(|y| sum[y][w - 1]).min().unwrap()
}

fn solve(file: File) -> io::Result<String> {
    let mat = read_matrix(file)?;
    Ok(minimal_path_sum(mat).to_string())
}

common::problem!("260324", "p082_matrix.txt", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn five_by_five() {
        let mat = vec![
            vec![131, 673, 234, 103, 18],
            vec![201, 96, 342, 965, 150],
            vec![630, 803, 746, 422, 111],
            vec![537, 699, 497, 121, 956],
            vec![805, 732, 524, 37, 331],
        ];
        assert_eq!(994, super::minimal_path_sum(mat));
    }
}
