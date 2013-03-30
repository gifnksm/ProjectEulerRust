pub struct Problem<'self> {
    id: uint,
    answer: &'self str,
    solver: extern fn() -> ~str
}
