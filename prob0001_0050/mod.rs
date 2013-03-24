pub mod prob0001;
pub mod prob0002;
pub mod prob0003;
pub mod prob0004;
pub mod prob0005;
pub mod prob0006;
pub mod prob0007;
pub mod prob0008;
pub mod prob0009;
pub mod prob0010;
pub mod prob0011;
pub mod prob0012;
pub mod prob0013;
pub mod prob0014;
pub mod prob0015;
pub mod prob0016;
pub mod prob0017;
pub mod prob0018;
pub mod prob0019;
pub mod prob0020;
pub mod prob0021;
pub mod prob0022;
pub mod prob0023;
pub mod prob0024;
pub mod prob0025;
pub mod prob0026;
pub mod prob0027;
pub mod prob0028;
pub mod prob0029;
pub mod prob0030;
pub mod prob0031;
pub mod prob0032;
pub mod prob0033;
pub mod prob0034;
pub mod prob0035;
pub mod prob0036;
pub mod prob0037;
pub mod prob0038;
pub mod prob0039;
pub mod prob0040;
pub mod prob0041;
pub mod prob0042;
pub mod prob0043;
pub mod prob0044;
pub mod prob0045;
pub mod prob0046;
pub mod prob0047;
pub mod prob0048;
pub mod prob0049;
pub mod prob0050;

priv use common::problem::{ Solver, Problem, UintProblem, IntProblem };

macro_rules! define_solver(
    ($module:ident, $num:expr, $answer:expr) => (
        define_solver!($module, $num, $answer, UintProblem)
    );
    ($module:ident, $num:expr, $answer:expr, $problem:ident) => (
        $problem(Solver {
            number: $num,
            answer: $answer,
            solver: $module::solve
        })
    );
)

pub static problems: &'static [Problem] = &[
    define_solver!(prob0001, 1, 233168),
    define_solver!(prob0002, 2, 4613732),
    define_solver!(prob0003, 3, 6857),
    define_solver!(prob0004, 4, 906609),
    define_solver!(prob0005, 5, 232792560),
    define_solver!(prob0006, 6, 25164150),
    define_solver!(prob0007, 7, 104743),
    define_solver!(prob0008, 8, 40824),
    define_solver!(prob0009, 9, 31875000),
    define_solver!(prob0010, 10, 142913828922),
    define_solver!(prob0011, 11, 70600674),
    define_solver!(prob0012, 12, 76576500),
    define_solver!(prob0013, 13, 5537376230),
    define_solver!(prob0014, 14, 837799),
    define_solver!(prob0015, 15, 137846528820),
    define_solver!(prob0016, 16, 1366),
    define_solver!(prob0017, 17, 21124),
    define_solver!(prob0018, 18, 1074),
    define_solver!(prob0019, 19, 171),
    define_solver!(prob0020, 20, 648),
    define_solver!(prob0021, 21, 31626),
    define_solver!(prob0022, 22, 871198282),
    define_solver!(prob0023, 23, 4179871),
    define_solver!(prob0024, 24, 2783915460),
    define_solver!(prob0025, 25, 4782),
    define_solver!(prob0026, 26, 983),
    define_solver!(prob0027, 27, -59231, IntProblem),
    define_solver!(prob0028, 28, 669171001),
    define_solver!(prob0029, 29, 9183),
    define_solver!(prob0030, 30, 443839),
    define_solver!(prob0031, 31, 73682),
    define_solver!(prob0032, 32, 45228),
    define_solver!(prob0033, 33, 100),
    define_solver!(prob0034, 34, 40730),
    define_solver!(prob0035, 35, 55),
    define_solver!(prob0036, 36, 872187),
    define_solver!(prob0037, 37, 748317),
    define_solver!(prob0038, 38, 932718654),
    define_solver!(prob0039, 39, 840),
    define_solver!(prob0040, 40, 210),
    define_solver!(prob0041, 41, 7652413),
    define_solver!(prob0042, 42, 162),
    define_solver!(prob0043, 43, 16695334890),
    define_solver!(prob0044, 44, 5482660),
    define_solver!(prob0045, 45, 1533776805),
    define_solver!(prob0046, 46, 5777),
    define_solver!(prob0047, 47, 134043),
    define_solver!(prob0048, 48, 9110846700),
    define_solver!(prob0049, 49, 296962999629),
    define_solver!(prob0050, 50, 997651)
];
