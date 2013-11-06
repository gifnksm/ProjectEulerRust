#[link(name = "prob0096", vers = "0.0")];
#[crate_type = "lib"];

use std::{char, iter, vec};
use std::io::buffered::BufferedReader;
use std::io::File;
use std::num::ToStrRadix;

pub static EXPECTED_ANSWER: &'static str = "24702";

static BOARD_WIDTH: uint = 9;
static BOARD_HEIGHT: uint = 9;
static GROUP_WIDTH: uint = 3;
static GROUP_HEIGHT: uint = 3;
static MAX_NUMBER: uint = 9;
type BITS = u16;
static MASK_ALL: BITS = 0x1ff;

struct SuDoku {
    name: ~str,
    map: [[BITS, .. BOARD_WIDTH], .. BOARD_HEIGHT]
}

impl TotalEq for SuDoku {
    #[inline(always)]
    fn equals(&self, other: &SuDoku) -> bool {
        self.name == other.name && range(0, BOARD_HEIGHT).all(|y| self.map[y] == other.map[y])
    }
}

impl Eq for SuDoku {
    #[inline(always)]
    fn eq(&self, other: &SuDoku) -> bool { self.equals(other) }
    #[inline(always)]
    fn ne(&self, other: &SuDoku) -> bool { !self.equals(other) }
}

impl ToStr for SuDoku {
    fn to_str(&self) -> ~str {
        let rows = do self.map.map |row| {
            let cells = do row.map |&cell| {
                if cell.population_count() == 1 {
                    ~"_"
                } else {
                    (1u << cell.trailing_zeros()).to_str()
                }
            };
            cells.concat()
        };
        self.name + "\n" + rows.connect("\n")
    }
}

impl Clone for SuDoku {
    fn clone(&self) -> SuDoku {
        SuDoku { name: self.name.clone(), map: self.map }
    }
}

impl SuDoku {
    fn get_num(&self, x: uint, y: uint) -> BITS {
        match self.map[y][x].population_count() {
            0 => -1,
            1 => self.map[y][x].trailing_zeros() + 1,
            _ => 0
        }
    }

    fn to_str_debug(&self) -> ~str {
        let row_strs = do vec::build(Some(BOARD_HEIGHT)) |push| {
            for y in range(0, BOARD_HEIGHT) {
                let cell_strs = do vec::build(Some(BOARD_WIDTH)) |push| {
                    for x in range(0, BOARD_WIDTH) {
                        let s = self.map[y][x].to_str_radix(2);
                        push(format!("{}:{}",
                                     self.get_num(x, y).to_str(),
                                     "0".repeat(MAX_NUMBER - s.len()) + s).replace("0", "_"));
                    }
                };
                push(cell_strs.connect(" "));
            }
        };
        self.name + "\n" + row_strs.connect("\n")
    }
}

fn read_sudoku<T: Reader>(r: &mut BufferedReader<T>) -> SuDoku {
    let mut sudoku = SuDoku {
        name: r.read_line().unwrap(),
        map: [[MASK_ALL, .. BOARD_WIDTH], .. BOARD_HEIGHT]
    };

    for y in range(0, BOARD_HEIGHT) {
        let line = r.read_line().unwrap();
        for x in range(0, BOARD_WIDTH) {
            let n = char::to_digit(line[x] as char, 10).unwrap();
            if n != 0 { sudoku.map[y][x] = 1 << (n - 1); }
        }
    }
    sudoku
}

fn solve_sudoku(mut puzzle: SuDoku) -> ~[SuDoku] {
    let group_it = range(0, GROUP_WIDTH * GROUP_HEIGHT)
        .map(|i| (i % GROUP_WIDTH, i / GROUP_WIDTH))
        .to_owned_vec();

    loop {
        let bkup = puzzle.clone();

        for y in range(0, BOARD_HEIGHT) {
            for x in range(0, BOARD_WIDTH) {
                if puzzle.map[y][x].population_count() != 1 { continue }

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
                let mut it = range(0, BOARD_WIDTH)
                    .filter(|&x| puzzle.map[y][x] & bit != 0);
                let next = it.next();
                if next.is_none() || it.next().is_some() { continue }
                puzzle.map[y][next.unwrap()] = bit;
            }

            for x in range(0, BOARD_WIDTH) {
                let mut it = range(0, BOARD_HEIGHT)
                    .filter(|&y| puzzle.map[y][x] & bit != 0);
                let next = it.next();
                if next.is_none() || it.next().is_some() { continue }
                puzzle.map[next.unwrap()][x] = bit;
            }

            for y0 in iter::range_step(0, BOARD_HEIGHT, GROUP_WIDTH) {
                for x0 in iter::range_step(0, BOARD_WIDTH, GROUP_HEIGHT) {
                    let mut it = group_it
                        .iter()
                        .map(|&(dx, dy)| (x0 + dx, y0 + dy))
                        .filter(|&(x, y)| puzzle.map[y][x] & bit != 0);
                    let next = it.next();
                    if next.is_some() && it.next().is_none() {
                        let (x, y) = next.unwrap();
                        puzzle.map[y][x] = bit;
                    }
                }
            }
        }

        if puzzle == bkup { break }
    }

    let it = range(0, BOARD_HEIGHT * BOARD_WIDTH)
        .map(|i| (i % BOARD_WIDTH, i / BOARD_WIDTH))
        .map(|(x, y)| (x, y, puzzle.map[y][x].population_count()))
        .to_owned_vec();

    if it.iter().any(|&(_x, _y, cnt)| cnt == 0) { return ~[]; }
    if it.iter().all(|&(_x, _y, cnt)| cnt == 1) { return ~[puzzle]; }

    let (x, y, _cnt) = *it.iter()
        .filter(|& &(_x, _y, cnt)| cnt > 1)
        .min_by(|& &(_x, _y, cnt)| cnt)
        .unwrap();

    let mut answers = ~[];
    for n in range(0, MAX_NUMBER) {
        let bit = 1 << n;
        if puzzle.map[y][x] & bit == 0 { continue }

        let mut p2 = puzzle.clone();
        p2.map[y][x] = bit;
        answers.push_all(solve_sudoku(p2));
    }

    answers
}

pub fn solve() -> ~str {
    let mut br = BufferedReader::new(
        File::open(&Path::new("files/sudoku.txt")).expect("file not found."));

    let mut puzzles = ~[];
    while !br.eof() { puzzles.push(read_sudoku(&mut br)); }
    let mut answers = puzzles
        .move_iter()
        .map(solve_sudoku)
        .map(|ans| { assert_eq!(ans.len(), 1); ans[0] });
    let mut sum = 0;
    for ans in answers {
        sum += 100 * ans.get_num(0, 0) + 10 * ans.get_num(1, 0) + ans.get_num(2, 0);
    }
    sum.to_str()
}
