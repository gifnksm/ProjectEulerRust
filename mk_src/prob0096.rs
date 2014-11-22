#![crate_name = "prob0096"]
#![crate_type = "rlib"]

use std::{fmt, iter};
use std::num::Int;
use std::io::{BufferedReader, File};

pub const EXPECTED_ANSWER: &'static str = "24702";

const BOARD_WIDTH: uint = 9;
const BOARD_HEIGHT: uint = 9;
const GROUP_WIDTH: uint = 3;
const GROUP_HEIGHT: uint = 3;
const MAX_NUMBER: uint = 9;
type BITS = u16;
const MASK_ALL: BITS = 0x1ff;

struct SuDoku {
    name: String,
    map: [[BITS, .. BOARD_WIDTH], .. BOARD_HEIGHT]
}

// #7622 (rust): #[deriving(Eq, Eq, Clone)] cannnot be used
impl Eq for SuDoku {}

impl PartialEq for SuDoku {
    #[inline]
    fn eq(&self, other: &SuDoku) -> bool {
        self.name == other.name && range(0, BOARD_HEIGHT).all(|y| self.map[y] == other.map[y])
    }
}

impl fmt::Show for SuDoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "{}", self.name));

        for row in self.map.iter() {
            for cell in row.iter() {
                if cell.count_ones() == 1 {
                    try!(write!(f, "_"));
                } else {
                    try!(write!(f, "{}", 1u << cell.trailing_zeros()));
                }
            }
            try!(writeln!(f, ""));
        }

        Ok(())
    }
}

impl Clone for SuDoku {
    #[inline]
    fn clone(&self) -> SuDoku {
        SuDoku { name: self.name.clone(), map: self.map }
    }
}

impl SuDoku {
    #[inline]
    fn get_num(&self, x: uint, y: uint) -> BITS {
        match self.map[y][x].count_ones() {
            0 => -1,
            1 => (self.map[y][x].trailing_zeros() + 1) as BITS,
            _ => 0
        }
    }
}

fn read_sudoku<T: Reader>(r: &mut BufferedReader<T>) -> Option<SuDoku> {
    r.read_line().ok()
        .and_then(|name| {
            let mut sudoku = SuDoku {
                name: name,
                map: [[MASK_ALL, .. BOARD_WIDTH], .. BOARD_HEIGHT]
            };
            for y in range(0, BOARD_HEIGHT) {
                match r.read_line().ok() {
                    None => return None,
                    Some(line) => {
                        for x in range(0, BOARD_WIDTH) {
                            let n = line.char_at(x).to_digit(10).unwrap();
                            if n != 0 { sudoku.map[y][x] = 1 << (n - 1); }
                        }
                    }
                }
            }
            Some(sudoku)
        })
}

fn solve_sudoku(mut puzzle: SuDoku) -> Vec<SuDoku> {
    let group_it = range(0, GROUP_WIDTH * GROUP_HEIGHT)
        .map(|i| (i % GROUP_WIDTH, i / GROUP_WIDTH))
        .collect::<Vec<(uint, uint)>>();

    loop {
        let bkup = puzzle.clone();

        for y in range(0, BOARD_HEIGHT) {
            for x in range(0, BOARD_WIDTH) {
                if puzzle.map[y][x].count_ones() != 1 { continue }

                let (x0, y0) = (x / GROUP_WIDTH * GROUP_WIDTH,
                                y / GROUP_HEIGHT * GROUP_HEIGHT);
                let row = range(0, BOARD_WIDTH).map(|x| (x, y));
                let col = range(0, BOARD_HEIGHT).map(|y| (x, y));
                let grp = group_it.iter().map(|&(dx, dy)| (x0 + dx, y0 + dy));

                let mut it = row.chain(col).chain(grp)
                    .filter(|&pos: &(uint, uint)| pos != (x, y));
                let mask = !puzzle.map[y][x] & MASK_ALL;
                for (x, y) in it { puzzle.map[y][x] &= mask; }
            }
        }

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

    let mut answers = Vec::new();
    for n in range(0, MAX_NUMBER) {
        let bit = 1 << n;
        if puzzle.map[y][x] & bit == 0 { continue }

        let mut p2 = puzzle.clone();
        p2.map[y][x] = bit;
        answers.push_all(solve_sudoku(p2).as_slice());
    }

    answers.into_iter().collect()
}

pub fn solve() -> String {
    let mut br = BufferedReader::new(
        File::open(&Path::new("files/p096_sudoku.txt")).ok().expect("file not found."));

    let mut puzzles = Vec::new();
    loop {
        match read_sudoku(&mut br) {
            Some(sudoku) => puzzles.push(sudoku),
            None => break
        }
    }
    let mut answers = puzzles
        .into_iter()
        .map(solve_sudoku)
        .map(|mut ans| { assert_eq!(ans.len(), 1); ans.remove(0).unwrap() });
    let mut sum = 0;
    for ans in answers {
        sum += 100 * ans.get_num(0, 0) + 10 * ans.get_num(1, 0) + ans.get_num(2, 0);
    }
    sum.to_string()
}
