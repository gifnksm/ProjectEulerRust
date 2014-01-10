pub struct Problem<'a> {
    id: uint,
    answer: &'a str,
    solve: extern fn() -> ~str
}
