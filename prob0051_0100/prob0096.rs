use core::num::{ ToStrRadix };
use core::iterator::{ IteratorUtil };

use common::extiter::{ ExtIteratorUtil, Range };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 96,
    answer: "24702",
    solver: solve
};

static WIDTH: uint = 9;
static HEIGHT: uint = 9;

struct SuDoku {
    name: ~str,
    map: [[u16, .. WIDTH], .. HEIGHT]
}

impl TotalEq for SuDoku {
    #[inline(always)]
    fn equals(&self, other: &SuDoku) -> bool {
        self.name == other.name && Range::new(0, HEIGHT).all(|&y| self.map[y] == other.map[y])
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
            str::concat(cells)
        };
        return self.name + "\n" + str::connect(rows, "\n");
    }
}

impl Clone for SuDoku {
    fn clone(&self) -> SuDoku {
        SuDoku { name: self.name.clone(), map: self.map }
    }
}

static MASK_ALL: u16 = 0x1ff;

impl SuDoku {
    fn get_num(&self, x: uint, y: uint) -> u16 {
        match self.map[y][x].population_count() {
            0 => -1,
            1 => self.map[y][x].trailing_zeros() + 1,
            _ => 0
        }
    }

    fn to_str_debug(&self) -> ~str {
        let row_strs = do vec::build_sized(HEIGHT) |push| {
            for uint::range(0, HEIGHT) |y| {
                let cell_strs = do vec::build_sized(WIDTH) |push| {
                    for uint::range(0, WIDTH) |x| {
                        let s = self.map[y][x].to_str_radix(2);
                        push(str::replace(fmt!("%s:%s",
                                               self.get_num(x, y).to_str(),
                                               str::repeat("0", 9 - s.len()) + s), "0", "_"));
                    }
                };
                push(str::connect(cell_strs, " "));
            }
        };
        return self.name + "\n" + str::connect(row_strs, "\n");
    }
}

fn read_sudoku<T: Reader>(r: T) -> SuDoku {
    let mut sudoku = SuDoku {
        name: r.read_line(),
        map: [[MASK_ALL, .. WIDTH], .. HEIGHT]
    };

    for uint::range(0, HEIGHT) |y| {
        let line = r.read_line();
        for uint::range(0, WIDTH) |x| {
            let n = char::to_digit(line[x] as char, 10).get();
            if n != 0 { sudoku.map[y][x] = 1 << (n - 1); }
        }
    }
    return sudoku;
}

fn solve_sudoku(mut puzzle: SuDoku) -> ~[SuDoku] {
    let mut answers = ~[];

    loop {
        let bkup = puzzle.clone();

        for uint::range(0, HEIGHT) |y| {
            for uint::range(0, WIDTH) |x| {
                if puzzle.map[y][x].population_count() != 1 { loop; }

                let bits = puzzle.map[y][x];
                for uint::range(0, WIDTH) |x2| {
                    if x2 != x { puzzle.map[y][x2] &= (!bits & MASK_ALL); }
                }
                for uint::range(0, HEIGHT) |y2| {
                    if y2 != y { puzzle.map[y2][x] &= (!bits & MASK_ALL); }
                }
                let x0 = x / 3 * 3, y0 = y / 3 * 3;
                for uint::range(x0, x0 + 3) |x2| {
                    for uint::range(y0, y0 + 3) |y2| {
                        if x2 != x && y2 != y { puzzle.map[y2][x2] &= (!bits & MASK_ALL); }
                    }
                }
            }
        }

        for uint::range(0, 9) |n| {
            let bit = 1 << n;

            for uint::range(0, HEIGHT) |y| {
                let it = Range::new(0, WIDTH).filter(|&x| puzzle.map[y][x] & bit != 0);
                if it.count_elem() != 1 { loop; }
                puzzle.map[y][it.first()] = bit;
            }

            for uint::range(0, WIDTH) |x| {
                let it = Range::new(0, HEIGHT).filter(|&y| puzzle.map[y][x] & bit != 0);
                if it.count_elem() != 1 { loop; }
                puzzle.map[it.first()][x] = bit;
            }

            for uint::range_step(0, HEIGHT, 3) |y0| {
                for uint::range_step(0, WIDTH, 3) |x0| {
                    let it = Range::new(0u, 3 * 3)
                        .transform(|i| (x0 + i % 3, y0 + i / 3))
                        .filter(|&(x, y)| puzzle.map[y][x] & bit != 0);
                    if it.count_elem() != 1 { loop; }
                    let (x, y) = it.first();
                    puzzle.map[y][x] = bit;
                }
            }
        }

        if puzzle == bkup {
            let it = Range::new(0, HEIGHT * WIDTH)
                .transform(|i| (i % WIDTH, i / WIDTH))
                .transform(|(x, y)| (x, y, puzzle.map[y][x].population_count()));

            if it.any(|&(_x, _y, cnt)| cnt == 0) { return ~[]; }
            if it.all(|&(_x, _y, cnt)| cnt == 1) { answers.push(puzzle); break; }

            let (x, y, cnt) = it.filter(|&(_x, _y, cnt)| cnt > 1)
                .min_as(|&(_x, _y, cnt)| cnt);

            for uint::range(0, 9) |n| {
                let bit = 1 << n;
                if puzzle.map[y][x] & bit == 0 { loop; }
                let mut p2 = puzzle.clone();
                p2.map[y][x] = bit;
                answers.push_all(solve_sudoku(p2));
            }
            break;
        }
    }

    return answers;
}

fn solve() -> ~str {
    let result = io::file_reader(&Path("files/sudoku.txt")).map(|&file| {
        let mut puzzles = ~[];
        while !file.eof() { puzzles.push(read_sudoku(file)); }
        puzzles
    }).map(|&puzzles| {
        let mut answers = ~[];
        for puzzles.each |&p| {
            let mut ans = solve_sudoku(p);
            assert_eq!(ans.len(), 1);
            answers.push(ans.pop());
        }
        answers
    }).map(|&answers| {
        let mut sum = 0;
        for answers.each |ans| {
            sum += 100 * ans.get_num(0, 0) + 10 * ans.get_num(1, 0) + ans.get_num(2, 0);
        }
        sum
    });

    match result {
        Err(msg) => fail!(msg),
        Ok(value) => return value.to_str()
    }
}
