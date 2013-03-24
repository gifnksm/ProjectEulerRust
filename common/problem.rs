pub struct Problem {
    id: uint,
    answer: &'self str,
    solver: extern fn() -> ~str
}
