extern mod std;
extern mod euler;

struct Solver<T> {
    number: uint,
    answer: T,
    solver: extern fn() -> T
}

impl<T: Eq> Solver<T> {
    fn solve(&self) {
        let answer = (self.solver)();
        assert_eq!(&answer, &self.answer);
        io::println(fmt!("Problem #%u: %?", self.number, self.answer));
    }
}

enum Wrap {
    Uint(Solver<uint>),
    Int(Solver<int>)
}

impl Wrap {
    fn solve(&self) {
        match self {
            &Uint(s) => s.solve(),
            &Int(s) => s.solve()
        }
    }
}

macro_rules! define_solver(
    ($module:ident, $num:expr, $answer:expr, $wrap:ident) => (
        $wrap(Solver {
            number: $num,
            answer: $answer,
            solver: $module::solve
        })
    )
)

#[path="./0001-0050/mod.rs"]
mod prob0001_0050;

use prob0001_0050::*;

static solvers: &'static [Wrap] = &[
    define_solver!(prob0001, 1, 233168, Uint),
    define_solver!(prob0002, 2, 4613732, Uint),
    define_solver!(prob0003, 3, 6857, Uint),
    define_solver!(prob0004, 4, 906609, Uint),
    define_solver!(prob0005, 5, 232792560, Uint),
    define_solver!(prob0006, 6, 25164150, Uint),
    define_solver!(prob0007, 7, 104743, Uint),
    define_solver!(prob0008, 8, 40824, Uint),
    define_solver!(prob0009, 9, 31875000, Uint),
    define_solver!(prob0010, 10, 142913828922, Uint),
    define_solver!(prob0011, 11, 70600674, Uint),
    define_solver!(prob0012, 12, 76576500, Uint),
    define_solver!(prob0013, 13, 5537376230, Uint),
    define_solver!(prob0014, 14, 837799, Uint),
    define_solver!(prob0015, 15, 137846528820, Uint),
    define_solver!(prob0016, 16, 1366, Uint),
    define_solver!(prob0017, 17, 21124, Uint),
    define_solver!(prob0018, 18, 1074, Uint),
    define_solver!(prob0019, 19, 171, Uint),
    define_solver!(prob0020, 20, 648, Uint),
    define_solver!(prob0021, 21, 31626, Uint),
    define_solver!(prob0022, 22, 871198282, Uint),
    define_solver!(prob0023, 23, 4179871, Uint),
    define_solver!(prob0024, 24, 2783915460, Uint),
    define_solver!(prob0025, 25, 4782, Uint),
    define_solver!(prob0026, 26, 983, Uint),
    define_solver!(prob0027, 27, -59231, Int),
    define_solver!(prob0028, 28, 669171001, Uint),
    define_solver!(prob0029, 29, 9183, Uint),
    define_solver!(prob0030, 30, 443839, Uint),
    define_solver!(prob0031, 31, 73682, Uint),
    define_solver!(prob0032, 32, 45228, Uint),
    define_solver!(prob0033, 33, 100, Uint),
    define_solver!(prob0034, 34, 40730, Uint),
    define_solver!(prob0035, 35, 55, Uint),
    define_solver!(prob0036, 36, 872187, Uint),
    define_solver!(prob0037, 37, 748317, Uint),
    define_solver!(prob0038, 38, 932718654, Uint),
    define_solver!(prob0039, 39, 840, Uint),
    define_solver!(prob0040, 40, 210, Uint),
    define_solver!(prob0041, 41, 7652413, Uint),
    define_solver!(prob0042, 42, 162, Uint),
    define_solver!(prob0043, 43, 16695334890, Uint),
    define_solver!(prob0044, 44, 5482660, Uint),
    define_solver!(prob0045, 45, 1533776805, Uint),
    define_solver!(prob0046, 46, 5777, Uint),
    define_solver!(prob0047, 47, 134043, Uint),
    define_solver!(prob0048, 48, 9110846700, Uint),
    define_solver!(prob0049, 49, 296962999629, Uint),
    define_solver!(prob0050, 50, 997651, Uint)
 ];

fn main() {
    for solvers.each |s| { s.solve(); };
}
