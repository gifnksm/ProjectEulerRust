#[link(name = "prob0084", vers = "0.0")];
#[crate_type = "lib"];

extern mod extra;

use std::{uint, vec};
use extra::sort;

pub static EXPECTED_ANSWER: &'static str = "101524";

pub enum Square {
    GO, A1, CC1, A2, T1, R1, B1, CH1, B2, B3, JAIL,
    C1, U1, C2, C3, R2, D1, CC2, D2, D3, FP,
    E1, CH2, E2, E3, R3, F1, F2, U2, F3, G2J,
    G1, G2, CC3, G3, R4, CH3, H1, T2, H2,
    NumSquare
}

impl Square {
    #[inline(always)]
    fn to_uint(&self) -> uint { *self as uint }

    #[inline(always)]
    fn from_uint(n: uint) -> Square {
        match n {
            0 => GO, 1 => A1, 2 => CC1, 3 => A2, 4 => T1,
            5 => R1, 6 => B1, 7 => CH1, 8 => B2, 9 => B3,
            10 => JAIL, 11 => C1, 12 => U1, 13 => C2, 14 => C3,
            15 => R2, 16 => D1, 17 => CC2, 18 => D2, 19 => D3,
            20 => FP, 21 => E1, 22 => CH2, 23 => E2, 24 => E3,
            25 => R3, 26 => F1, 27 => F2, 28 => U2, 29 => F3,
            30 => G2J, 31 => G1, 32 => G2, 33 => CC3, 34 => G3,
            35 => R4, 36 => CH3, 37 => H1, 38 => T2, 39 => H2,
            _ => fail!()
        }
    }
}

static NUM_SQUARE: uint = NumSquare as uint;

#[inline(always)]
fn create_mat<T>(h: uint, w: uint, f: &fn(uint, uint) -> T) -> ~[~[T]] {
    do vec::from_fn(h) |i| { do vec::from_fn(w) |j| { f(i, j) } }
}

#[inline(always)]
pub fn mul_mat(m1: &[~[float]], m2: &[~[float]]) -> ~[~[float]] {
    do create_mat(m1.len(), m2[0].len()) |i, j| {
        let mut sum = 0f;
        for uint::range(0, m1[0].len()) |k| {
            sum += m1[i][k] * m2[k][j];
        }
        sum
    }
}

#[inline(always)]
pub fn add_mat(m1: &[~[float]], m2: &[~[float]]) -> ~[~[float]] {
    do create_mat(m1.len(), m1[0].len()) |i, j| { m1[i][j] + m2[i][j] }
}

#[inline(always)]
pub fn sub_mat(m1: &[~[float]], m2: &[~[float]]) -> ~[~[float]] {
    do create_mat(m1.len(), m1[0].len()) |i, j| { m1[i][j] - m2[i][j] }
}

#[inline(always)]
pub fn trans_mat(m: &[~[float]]) -> ~[~[float]] {
    do create_mat(m[0].len(), m.len()) |i, j| { m[j][i] }
}

fn print_mat(mat: &[~[float]]) {
    print("     ");
    for uint::range(0, mat[0].len()) |j| {
        print(fmt!(" %-4? ", Square::from_uint(j % NUM_SQUARE)));
    }
    println("");
    for uint::range(0, mat.len()) |i| {
        let mut sum = 0f;
        print(fmt!("%-4? ", Square::from_uint(i % NUM_SQUARE)));
        for uint::range(0, mat[i].len()) |j| {
            if mat[i][j] == 0f {
                print("0     ");
            } else{
                print(fmt!("%.3f ", mat[i][j]));
            }
            sum += mat[i][j];
        }
        println(fmt!("| %.3f", sum));
    }

    print("     ");
    for mat[0].len().times {
        print("----- ")
    }
    println("");
    print("     ");
    for uint::range(0, mat[0].len()) |j| {
        let mut sum = 0f;
        for uint::range(0, mat.len()) |i| {
            sum += mat[i][j];
        }
        print(fmt!("%.3f ", sum));
    }
    println("");
}

fn create_roll_map(dice_side: uint) -> ~[(float, float)] {
    let mut map = vec::from_elem(dice_side * 2 + 1, (0, 0));
    for uint::range(1, dice_side + 1) |i| {
        for uint::range(i, dice_side + 1) |j| {
            let sum = i + j;
            let (p, q) = map[sum];
            map[sum] = if i == j { (p, q + 1) } else { (p + 2, q) };
        }
    }

    let cases = dice_side * dice_side;
    return do map.map |&(p, q)| {
        ((p as float) / (cases as float), (q as float) / (cases as float))
    };
}

fn get_trans(roll_map: &[(float, float)]) -> ~[~[float]] {
    let trans_singles = do create_mat(NUM_SQUARE, NUM_SQUARE) |dst, src| {
        let diff = (dst + NUM_SQUARE - src) % NUM_SQUARE;
        if diff < roll_map.len() { roll_map[diff].first() } else { 0f }
    };
    let trans_doubles = do create_mat(NUM_SQUARE, NUM_SQUARE) |dst, src| {
        let diff = (dst + NUM_SQUARE - src) % NUM_SQUARE;
        if diff < roll_map.len() { roll_map[diff].second() } else { 0f }
    };

    let trans_cc = do create_mat(NUM_SQUARE, NUM_SQUARE) |dst, src| {
        match Square::from_uint(src) {
            CC1 | CC2 | CC3 => {
                match Square::from_uint(dst) {
                    GO   => 1f / 16f,
                    JAIL => 1f / 16f,
                    _ => if src == dst { 14f / 16f } else { 0f }
                }
            },
            _ => { if src == dst { 1f } else { 0f } }
        }
    };

    let trans_ch = do create_mat(NUM_SQUARE, NUM_SQUARE) |dst, src| {
        match Square::from_uint(src) {
            ch @ CH1 | ch @ CH2 | ch @ CH3 => {
                match Square::from_uint(dst) {
                    GO => 1f / 16f,
                    JAIL => 1f / 16f,
                    // Go back 3 square
                    CC3 => match ch { CH3 => 1f / 16f, _ => 0f},
                    // Go back 3 square
                    D2 => match ch { CH2 => 1f / 16f, _ => 0f },
                    // Go back 3 square
                    T1 => match ch { CH1 => 1f / 16f, _ => 0f },
                    C1 => 1f / 16f,
                    E3 => 1f / 16f,
                    H2 => 1f / 16f,
                    // Go to next R + Go to R1
                    R1 => match ch { CH3 => 3f / 16f, _ => 1f / 16f },
                    // Go to next R only
                    R2 => match ch { CH1 => 2f / 16f, _ => 0f },
                    // Go to next R only
                    R3 => match ch { CH2 => 2f / 16f, _ => 0f },
                    // Go to next U
                    U1 => match ch { CH1 | CH3 => 1f / 16f, _ => 0f },
                    // Go to next U
                    U2 => match ch { CH2 => 1f / 16f, _ => 0f },
                    _  if dst == src => 6f / 16f,
                    _  => 0f
                }
            },
            _ => { if dst == src { 1f } else { 0f } }
        }
    };

    let trans_g2j = do create_mat(NUM_SQUARE, NUM_SQUARE) |dst, src| {
        match Square::from_uint(src) {
            G2J => match Square::from_uint(dst) { JAIL => 1f, _ => 0f },
            _ => { if src == dst { 1f } else { 0f } }
        }
    };

    let trans_all_g2j = do create_mat(NUM_SQUARE, NUM_SQUARE) |dst, _src| {
        match Square::from_uint(dst) { JAIL => 1f, _ => 0f }
    };

    let trans_square = mul_mat(mul_mat(trans_cc, trans_ch), trans_g2j);
    let trans_sq_singles  = mul_mat(trans_square, trans_singles);
    let trans_sq_doubles  = mul_mat(trans_square, trans_doubles);
    let trans_g2j_doubles = mul_mat(trans_all_g2j, trans_doubles);

    return do create_mat(NUM_SQUARE * 3, NUM_SQUARE * 3) |i, j| {
        let dst_block = i / NUM_SQUARE;
        let dst = i % NUM_SQUARE;
        let src_block = j / NUM_SQUARE;
        let src = j % NUM_SQUARE;

        match (dst_block, src_block) {
            (0, 0) => trans_sq_singles[dst][src],
            (1, 0) => trans_sq_doubles[dst][src],
            (0, 1) => trans_sq_singles[dst][src],
            (2, 1) => trans_sq_doubles[dst][src],
            (0, 2) => trans_sq_singles[dst][src] + trans_g2j_doubles[dst][src],
            _      => 0f
        }
    }

    // return add_mat(
    //     mul_mat(trans_square, trans_singles),
    //     mul_mat(
    //         add_mat(
    //             mul_mat(trans_square, trans_singles),
    //             mul_mat(
    //                 add_mat(
    //                     mul_mat(trans_square, trans_singles),
    //                     mul_mat(trans_all_g2j, trans_doubles)),
    //                 mul_mat(trans_square, trans_doubles))),
    //         mul_mat(trans_square, trans_doubles)));
}

pub fn solve() -> ~str {
    let trans = get_trans(create_roll_map(4));
    let mut vec = do create_mat(trans.len(), 1) |i, _j| {
        if i == 0 { 1f } else { 0f }
    };

    loop {
        let vec2 = mul_mat(trans, vec);
        let sub = sub_mat(vec2, vec);
        let err = mul_mat(trans_mat(sub), sub);
        if err[0][0] < 1e-10 { break; }
        vec = vec2;
    }

    let mut pairs = do vec::from_fn(NUM_SQUARE) |i| {
        (0f, Square::from_uint(i))
    };
    for vec.iter().enumerate().advance |(i, vs)| {
        let dst = i % NUM_SQUARE;
        let (p, sq) = pairs[dst];
        pairs[dst] = (p + vs[0], sq);
    }
    sort::quick_sort(pairs, |&(p1, _), &(p2, _)| p1 >= p2);
    return pairs.slice(0, 3).map(|&(_, sq)| { fmt!("%02u", sq.to_uint()) }).concat();
}

#[cfg(test)]
mod test {
    use super::*;
    use std::uint;

    #[test]
    fn test_int_convertible_square() {
        for uint::range(0, 40) |n| {
            let sq = Square::from_uint(n);
            assert_eq!(sq.to_uint(), n);
        }
    }

    #[test]
    fn test_mul_mat() {
        assert_eq!(mul_mat([~[1f, 2f, 3f]], [~[1f], ~[2f], ~[3f]]),
                   ~[~[14f]]);
        assert_eq!(mul_mat([~[1f, 2f, 3f], ~[4f, 5f, 6f], ~[7f, 8f, 9f]],
                           [~[1f], ~[0f], ~[0f]]),
                   ~[~[1f], ~[4f], ~[7f]]);
        assert_eq!(mul_mat([~[1f, 2f, 3f], ~[4f, 5f, 6f], ~[7f, 8f, 9f]],
                           [~[1f, 0f], ~[0f, 0f], ~[0f, 1f]]),
                   ~[~[1f, 3f], ~[4f, 6f], ~[7f, 9f]]);
    }
}
