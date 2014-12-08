#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(slicing_syntax)]

extern crate common;

use std::io::{BufferedReader, File, IoErrorKind, IoResult};
use std::iter;
use std::num::Int;
use common::Solver;

const BOARD_WIDTH: uint = 9;
const BOARD_HEIGHT: uint = 9;
const GROUP_WIDTH: uint = 3;
const GROUP_HEIGHT: uint = 3;
const MAX_NUMBER: uint = 9;
type BITS = u16;
const MASK_ALL: BITS = 0x1ff;

#[deriving(Eq, PartialEq, Clone, Show)]
struct SuDoku {
    name: String,
    map: [[BITS, .. BOARD_WIDTH], .. BOARD_HEIGHT]
}

impl SuDoku {
    fn get_at(&self, x: uint, y: uint) -> uint {
        match self.map[y][x].count_ones() {
            0 => -1,
            1 => self.map[y][x].trailing_zeros() + 1,
            _ => 0
        }
    }

    fn set_at(&mut self, x: uint, y: uint, n: uint) {
        self.map[y][x] = 1 << (n - 1);
    }
}

fn read_sudoku<T: Reader>(br: &mut BufferedReader<T>) -> IoResult<Option<SuDoku>> {
    let name = match br.read_line() {
        Ok(line) => line,
        Err(err) => {
            if err.kind == IoErrorKind::EndOfFile {
                return Ok(None)
            }
            return Err(err)
        }
    };
    let mut sudoku = SuDoku {
        name: name,
        map: [[MASK_ALL, .. BOARD_WIDTH], .. BOARD_HEIGHT]
    };

    for y in range(0, BOARD_HEIGHT) {
        let line = try!(br.read_line());
        for x in range(0, BOARD_WIDTH) {
            let n = line.char_at(x).to_digit(10).unwrap();
            if n != 0 { sudoku.set_at(x, y, n); }
        }
    }

    Ok(Some(sudoku))
}

fn solve_sudoku(mut puzzle: SuDoku) -> Vec<SuDoku> {
    let group_it = range(0, GROUP_WIDTH * GROUP_HEIGHT)
        .map(|i| (i % GROUP_WIDTH, i / GROUP_WIDTH))
        .collect::<Vec<_>>();

    loop {
        let bkup = puzzle.clone();

        // if the number on (x, y) is uniquely determined, off the number bit on the other cells.
        for y in range(0, BOARD_HEIGHT) {
            for x in range(0, BOARD_WIDTH) {
                if puzzle.map[y][x].count_ones() != 1 { continue }

                let (x0, y0) = ((x / GROUP_WIDTH) * GROUP_WIDTH,
                                (y / GROUP_HEIGHT) * GROUP_HEIGHT);
                let row = range(0, BOARD_WIDTH).map(|x| (x, y));
                let col = range(0, BOARD_HEIGHT).map(|y| (x, y));
                let grp = group_it.iter().map(|&(dx, dy)| (x0 + dx, y0 + dy));

                let mut it = row.chain(col).chain(grp)
                    .filter(|&pos: &(uint, uint)| pos != (x, y));
                let mask = !puzzle.map[y][x] & MASK_ALL;
                for (x, y) in it { puzzle.map[y][x] &= mask; }
            }
        }

        // if the number n can be appears on only one cell in the row or col or group,
        // the number of the cell is n.
        for n in range(0, MAX_NUMBER) {
            let bit = 1 << n;

            for y in range(0, BOARD_HEIGHT) {
                let next = {
                    let mut it = range(0, BOARD_WIDTH)
                        .filter(|&x| puzzle.map[y][x] & bit != 0);
                    let next = it.next();
                    if next.is_none() || it.next().is_some() { continue }
                    next
                };
                puzzle.map[y][next.unwrap()] = bit;
            }

            for x in range(0, BOARD_WIDTH) {
                let next = {
                    let mut it = range(0, BOARD_HEIGHT)
                        .filter(|&y| puzzle.map[y][x] & bit != 0);
                    let next = it.next();
                    if next.is_none() || it.next().is_some() { continue }
                    next
                };
                puzzle.map[next.unwrap()][x] = bit;
            }

            for y0 in iter::range_step(0, BOARD_HEIGHT, GROUP_WIDTH) {
                for x0 in iter::range_step(0, BOARD_WIDTH, GROUP_HEIGHT) {
                    let next = {
                        let mut it = group_it
                            .iter()
                            .map(|&(dx, dy)| (x0 + dx, y0 + dy))
                            .filter(|&(x, y)| puzzle.map[y][x] & bit != 0);
                        let next = it.next();
                        if next.is_none() || it.next().is_some() { continue }
                        next
                    };
                    let (x, y) = next.unwrap();
                    puzzle.map[y][x] = bit;
                }
            }
        }

        if puzzle == bkup { break }
    }

    let it = range(0, BOARD_HEIGHT * BOARD_WIDTH)
        .map(|i| (i % BOARD_WIDTH, i / BOARD_WIDTH))
        .map(|(x, y)| (x, y, puzzle.map[y][x].count_ones() as BITS))
        .collect::<Vec<(uint, uint, BITS)>>();

    if it.iter().any(|&(_x, _y, cnt)| cnt == 0) { return vec![]; }
    if it.iter().all(|&(_x, _y, cnt)| cnt == 1) { return vec![puzzle]; }

    let (x, y, _cnt) = *it.iter()
        .filter(|& &(_x, _y, cnt)| cnt > 1)
        .min_by(|& &(_x, _y, cnt)| cnt)
        .unwrap();

    let mut answers = vec![];
    for n in range(0, MAX_NUMBER) {
        let bit = 1 << n;
        if puzzle.map[y][x] & bit == 0 { continue }

        let mut p2 = puzzle.clone();
        p2.map[y][x] = bit;
        answers.extend(solve_sudoku(p2).into_iter());
    }
    answers
}

fn solve(file: File) -> IoResult<String> {
    let mut br = BufferedReader::new(file);

    let mut answers = Vec::new();
    while let Some(puzzle) = try!(read_sudoku(&mut br)) {
        let mut ans = solve_sudoku(puzzle);
        assert_eq!(1, ans.len());
        answers.push(ans.pop().unwrap())
    }

    let mut sum = 0;
    for ans in answers.iter() {
        sum += 100 * ans.get_at(0, 0) + 10 * ans.get_at(1, 0) + ans.get_at(2, 0);
    }

    Ok(sum.to_string())
}

fn main() {
    Solver::new_with_file("24702", "p096_sudoku.txt", solve).run();
}
