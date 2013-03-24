pub struct Problem {
    number: uint,
    answer: &'self str,
    solver: extern fn() -> ~str
}

pub impl Problem<'self> {
    fn solve(&self) {
        assert!(self.answer == (self.solver)());
        io::println(fmt!("Problem #%u: %?", self.number, self.answer));
    }
}
