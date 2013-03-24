use common::prime::{ Prime };

pub fn solve() -> ~str {
    let mut ps = Prime();
    return ps.get_at(10000).to_str();
}
