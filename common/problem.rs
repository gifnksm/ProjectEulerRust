pub struct Problem {
    number: uint,
    answer: &'self str,
    solver: extern fn() -> ~str
}
