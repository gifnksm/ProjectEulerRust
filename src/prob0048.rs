#[link(name = "prob0048", vers = "0.0", package_id = "prob0048")];
#[crate_type = "lib"];

pub static EXPECTED_ANSWER: &'static str = "9110846700";

fn pow_mod(base: uint, exponent: uint, modulo: uint) -> uint {
    if base == 0 { return 0; }
    let mut acc = 1;
    exponent.times(|| {
        acc = (acc * base) % modulo;
    });
    return acc;
}

pub fn solve() -> ~str {
    let modulo  = 100_0000_0000;
    let mut sum = 0;
    for n in range(1u, 1000 + 1) {
        sum = (sum + pow_mod(n, n, modulo)) % modulo;
    }
    return sum.to_str();
}
