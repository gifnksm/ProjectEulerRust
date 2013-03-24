use euler::prime::{ Prime };

pub fn solve() -> uint {
    let mut ps = Prime();
    return ps.get_at(10000);
}
