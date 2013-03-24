pub struct Solver<T> {
    number: uint,
    answer: T,
    solver: extern fn() -> T
}

pub impl<T: Eq> Solver<T> {
    fn solve(&self) {
        let answer = (self.solver)();
        assert_eq!(&answer, &self.answer);
        io::println(fmt!("Problem #%u: %?", self.number, self.answer));
    }
}

pub enum Problem {
    UintProblem(Solver<uint>),
    IntProblem(Solver<int>)
}

pub impl Problem {
    fn solve(&self) {
        match self {
            &UintProblem(s) => s.solve(),
            &IntProblem(s) => s.solve()
        }
    }

    fn number(&self) -> uint {
        match self {
            &UintProblem(s) => s.number,
            &IntProblem(s) => s.number
        }
    }
}
