#![crate_id = "prob0127"]
#![crate_type = "rlib"]

use std::iter;

pub static EXPECTED_ANSWER: &'static str = "18407904";

// [定理]
// a + b = c のとき、
// GCD(a, b) = 1 => GCD(a, c) = GCD(b, c) = 1
//
// [証明]
// まず、GCD(a, b) = 1 => GCD(a, c) = 1 を示す。
// GCD(a, b) のとき、GCD(a, c) = k > 1 であると仮定すると、
// 整数 n, m を用いて a = kn, c = km と表すことができる。
// b = c - a = km - kn = k(m, n) より、GCD(a, b) >= k となり矛盾。
// よって、GCD(a, c) = 1 である。
//
// 次に、GCD(a, b) = 1 => GCD(b, c) = 1 を示す。
// GCD(a, b) = 1 のとき、GCD(b, c) = k > 1 であると仮定すると、
// 整数 n, m を用いて b = kn, c = km と表すことができる。
// ここで、a = c - b = km - kn = k(m - n) より、 GCD(a, b) >= k となり矛盾。
// よって、GCD(b, c) = 1 である。

#[deriving(Eq, TotalEq, Ord, TotalOrd, Clone, Show)]
struct Rad(uint, uint, Vec<uint>); // (n, rad, facts)

fn create_rad_vec(n_limit: uint) -> Vec<Rad> {
    let mut rad_vec = Vec::from_fn(n_limit, |i| (1, i, Vec::new()));
    for p in range(2, rad_vec.len()) {
        let (rad_p, _, _) = *rad_vec.get(p);
        if rad_p != 1 { continue }

        for kp in iter::count(p, p).take_while(|&kp| kp < n_limit) {
            let (ref mut rad_kp, _, ref mut facts) = *rad_vec.get_mut(kp);
            (*rad_kp) *= p;
            facts.push(p);
        }
    }
    rad_vec.move_iter().map(|(x, y, z)| Rad(x, y, z)).collect()
}

fn rad_has_union(a: &[uint], b: &[uint]) -> bool {
    let mut i_a = 0;
    let mut i_b = 0;

    loop {
        if i_a >= a.len() || i_b >= b.len() { return false }
        match a[i_a].cmp(&b[i_b]) {
            Equal   => return true,
            Less    => i_a += 1,
            Greater => i_b += 1
        }
    }
}

fn abc_hits_c_sum(c_limit: uint) -> uint {
    let rad_vec = create_rad_vec(c_limit);
    let mut sorted_rad_vec = rad_vec.tail().to_owned(); // drop a == 0 element
    sorted_rad_vec.sort();

    let mut c_sum = 0;

    for c in range(3, c_limit) {
        let Rad(rad_c, _, ref c_facts) = *rad_vec.get(c);
        if rad_c == c { continue } // if rad(c) == c, rad(ab) must be 1. this doesn't satisfy condition 2.

        for &Rad(rad_a, a, ref a_facts) in sorted_rad_vec.iter() {
            if rad_a >= c / rad_c { break }
            if a >= (c + 1) / 2 { continue }

            let Rad(rad_b, _, _) = *rad_vec.get(c - a);
            let rad_abc = rad_a * rad_b * rad_c;
            if rad_abc >= c || (a != 1 && rad_has_union(c_facts.as_slice(), a_facts.as_slice())) { continue; }
            c_sum += c;
        }
    }

    c_sum
}

pub fn solve() -> StrBuf { abc_hits_c_sum(120000).to_str() }

#[cfg(test)]
mod tests {
    use super::Rad;

    #[test]
    fn create_rad_vec() {
        let rad_vec = vec![
            Rad(1, 0, vec![]), Rad(1, 1, vec![]), Rad(2, 2, vec![2]), Rad(3, 3, vec![3]), Rad(2, 4, vec![2]), Rad(5, 5, vec![5]),
            Rad(6, 6, vec![2, 3]), Rad(7, 7, vec![7]), Rad(2, 8, vec![2]), Rad(3, 9, vec![3])];
        assert_eq!(rad_vec, super::create_rad_vec(10))
    }

    #[test]
    fn rad_has_union() {
        assert!(super::rad_has_union(&[2], &[2]));
        assert!(!super::rad_has_union(&[2], &[4]));

        assert!(super::rad_has_union(&[3], &[2, 3]));
        assert!(super::rad_has_union(&[3, 5], &[2, 5]));

        assert!(!super::rad_has_union(&[2, 3, 5], &[7, 11, 13]));
    }

    #[test]
    fn abc_hits_c_sum() {
        assert_eq!(12523, super::abc_hits_c_sum(1000));
    }
}
