//! [Problem 96](https://projecteuler.net/problem=96) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
    usize,
};

const BOARD_WIDTH: usize = 9;
const BOARD_HEIGHT: usize = 9;
const GROUP_WIDTH: usize = 3;
const GROUP_HEIGHT: usize = 3;
const MAX_NUMBER: usize = 9;
type BITS = u16;
const MASK_ALL: BITS = 0x1ff;

#[derive(Eq, PartialEq, Clone, Debug)]
struct SuDoku {
    name: String,
    map: [[BITS; BOARD_WIDTH]; BOARD_HEIGHT],
}

impl SuDoku {
    fn get_at(&self, x: usize, y: usize) -> usize {
        match self.map[y][x].count_ones() {
            0 => usize::MAX,
            1 => (self.map[y][x].trailing_zeros() + 1) as usize,
            _ => 0,
        }
    }

    fn set_at(&mut self, x: usize, y: usize, n: usize) {
        self.map[y][x] = 1 << (n - 1);
    }
}

fn read_sudoku<T: Read>(br: &mut BufReader<T>) -> io::Result<Option<SuDoku>> {
    let mut line = String::new();
    if br.read_line(&mut line)? == 0 {
        return Ok(None);
    }

    let mut sudoku = SuDoku {
        name: line.trim().to_string(),
        map: [[MASK_ALL; BOARD_WIDTH]; BOARD_HEIGHT],
    };

    for (y, line) in br.lines().enumerate().take(BOARD_HEIGHT) {
        let line = line?;
        for (x, c) in line.chars().enumerate().take(BOARD_WIDTH) {
            let n = c.to_digit(10).unwrap();
            if n != 0 {
                sudoku.set_at(x, y, n as usize);
            }
        }
    }

    Ok(Some(sudoku))
}

fn solve_sudoku(mut puzzle: SuDoku) -> Vec<SuDoku> {
    let group_it = (0..(GROUP_WIDTH * GROUP_HEIGHT))
        .map(|i| (i % GROUP_WIDTH, i / GROUP_WIDTH))
        .collect::<Vec<_>>();

    loop {
        let bkup = puzzle.clone();

        // if the number on (x, y) is uniquely determined, off the number bit on the other cells.
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                if puzzle.map[y][x].count_ones() != 1 {
                    continue;
                }

                let (x0, y0) = (
                    (x / GROUP_WIDTH) * GROUP_WIDTH,
                    (y / GROUP_HEIGHT) * GROUP_HEIGHT,
                );
                let row = (0..BOARD_WIDTH).map(|x| (x, y));
                let col = (0..BOARD_HEIGHT).map(|y| (x, y));
                let grp = group_it.iter().map(|&(dx, dy)| (x0 + dx, y0 + dy));

                let it = row
                    .chain(col)
                    .chain(grp)
                    .filter(|&pos: &(usize, usize)| pos != (x, y));
                let mask = !puzzle.map[y][x] & MASK_ALL;
                for (x, y) in it {
                    puzzle.map[y][x] &= mask;
                }
            }
        }

        // if the number n can be appears on only one cell in the row or col or group,
        // the number of the cell is n.
        for n in 0..MAX_NUMBER {
            let bit = 1 << n;

            for y in 0..BOARD_HEIGHT {
                let next = {
                    let mut it = (0..BOARD_WIDTH).filter(|&x| puzzle.map[y][x] & bit != 0);
                    let next = it.next();
                    if next.is_none() || it.next().is_some() {
                        continue;
                    }
                    next
                };
                puzzle.map[y][next.unwrap()] = bit;
            }

            for x in 0..BOARD_WIDTH {
                let next = {
                    let mut it = (0..BOARD_HEIGHT).filter(|&y| puzzle.map[y][x] & bit != 0);
                    let next = it.next();
                    if next.is_none() || it.next().is_some() {
                        continue;
                    }
                    next
                };
                puzzle.map[next.unwrap()][x] = bit;
            }

            for y0 in (0..BOARD_HEIGHT).step_by(GROUP_HEIGHT) {
                for x0 in (0..BOARD_WIDTH).step_by(GROUP_WIDTH) {
                    let next = {
                        let mut it = group_it
                            .iter()
                            .map(|&(dx, dy)| (x0 + dx, y0 + dy))
                            .filter(|&(x, y)| puzzle.map[y][x] & bit != 0);
                        let next = it.next();
                        if next.is_none() || it.next().is_some() {
                            continue;
                        }
                        next
                    };
                    let (x, y) = next.unwrap();
                    puzzle.map[y][x] = bit;
                }
            }
        }

        if puzzle == bkup {
            break;
        }
    }

    let it = (0..(BOARD_HEIGHT * BOARD_WIDTH))
        .map(|i| (i % BOARD_WIDTH, i / BOARD_WIDTH))
        .map(|(x, y)| (x, y, puzzle.map[y][x].count_ones() as BITS))
        .collect::<Vec<(usize, usize, BITS)>>();

    if it.iter().any(|&(_x, _y, cnt)| cnt == 0) {
        return vec![];
    }
    if it.iter().all(|&(_x, _y, cnt)| cnt == 1) {
        return vec![puzzle];
    }

    let (x, y, _cnt) = *it
        .iter()
        .filter(|&&(_x, _y, cnt)| cnt > 1)
        .min_by_key(|&&(_x, _y, cnt)| cnt)
        .unwrap();

    let mut answers = vec![];
    for n in 0..MAX_NUMBER {
        let bit = 1 << n;
        if puzzle.map[y][x] & bit == 0 {
            continue;
        }

        let mut p2 = puzzle.clone();
        p2.map[y][x] = bit;
        answers.extend(solve_sudoku(p2).into_iter());
    }
    answers
}

fn solve(file: File) -> io::Result<String> {
    let mut br = BufReader::new(file);

    let mut answers = Vec::new();
    while let Some(puzzle) = read_sudoku(&mut br)? {
        let mut ans = solve_sudoku(puzzle);
        assert_eq!(1, ans.len());
        answers.push(ans.pop().unwrap())
    }

    let mut sum = 0;
    for ans in &answers {
        sum += 100 * ans.get_at(0, 0) + 10 * ans.get_at(1, 0) + ans.get_at(2, 0);
    }

    Ok(sum.to_string())
}

common::problem!("24702", "p096_sudoku.txt", solve);
