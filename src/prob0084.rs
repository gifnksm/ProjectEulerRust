#[crate_type = "rlib"];

#[feature(globs)];

use std::vec;

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
fn create_mat<T>(h: uint, w: uint, f: |uint, uint| -> T) -> ~[~[T]] {
    vec::from_fn(h, |i| vec::from_fn(w, |j| f(i, j)))
}

#[inline(always)]
pub fn mul_mat(m1: &[~[f64]], m2: &[~[f64]]) -> ~[~[f64]] {
    create_mat(m1.len(), m2[0].len(), |i, j| {
        let mut sum = 0.0;
        for k in range(0, m1[0].len()) {
            sum += m1[i][k] * m2[k][j];
        }
        sum
    })
}

#[inline(always)]
pub fn add_mat(m1: &[~[f64]], m2: &[~[f64]]) -> ~[~[f64]] {
    create_mat(m1.len(), m1[0].len(), |i, j| { m1[i][j] + m2[i][j] })
}

#[inline(always)]
pub fn sub_mat(m1: &[~[f64]], m2: &[~[f64]]) -> ~[~[f64]] {
    create_mat(m1.len(), m1[0].len(), |i, j| { m1[i][j] - m2[i][j] })
}

#[inline(always)]
pub fn trans_mat(m: &[~[f64]]) -> ~[~[f64]] {
    create_mat(m[0].len(), m.len(), |i, j| { m[j][i] })
}

fn create_roll_map(dice_side: uint) -> ~[(f64, f64)] {
    let mut map = vec::from_elem(dice_side * 2 + 1, (0, 0));
    for i in range(1, dice_side + 1) {
        for j in range(i, dice_side + 1) {
            let sum = i + j;
            let (p, q) = map[sum];
            map[sum] = if i == j { (p, q + 1) } else { (p + 2, q) };
        }
    }

    let cases = dice_side * dice_side;
    map.map(|&(p, q)| ((p as f64) / (cases as f64), (q as f64) / (cases as f64)))
}

fn get_trans(roll_map: &[(f64, f64)]) -> ~[~[f64]] {
    let trans_singles = create_mat(NUM_SQUARE, NUM_SQUARE, |dst, src| {
        let diff = (dst + NUM_SQUARE - src) % NUM_SQUARE;
        if diff < roll_map.len() { roll_map[diff].first() } else { 0.0 }
    });
    let trans_doubles = create_mat(NUM_SQUARE, NUM_SQUARE, |dst, src| {
        let diff = (dst + NUM_SQUARE - src) % NUM_SQUARE;
        if diff < roll_map.len() { roll_map[diff].second() } else { 0.0 }
    });

    let trans_cc = create_mat(NUM_SQUARE, NUM_SQUARE, |dst, src| {
        match Square::from_uint(src) {
            CC1 | CC2 | CC3 => {
                match Square::from_uint(dst) {
                    GO   => 1.0 / 16.0,
                    JAIL => 1.0 / 16.0,
                    _ => if src == dst { 14.0 / 16.0 } else { 0.0 }
                }
            },
            _ => { if src == dst { 1.0 } else { 0.0 } }
        }
    });

    let trans_ch = create_mat(NUM_SQUARE, NUM_SQUARE, |dst, src| {
        match Square::from_uint(src) {
            ch @ CH1 | ch @ CH2 | ch @ CH3 => {
                match Square::from_uint(dst) {
                    GO => 1.0 / 16.0,
                    JAIL => 1.0 / 16.0,
                    // Go back 3 square
                    CC3 => match ch { CH3 => 1.0 / 16.0, _ => 0.0},
                    // Go back 3 square
                    D2 => match ch { CH2 => 1.0 / 16.0, _ => 0.0 },
                    // Go back 3 square
                    T1 => match ch { CH1 => 1.0 / 16.0, _ => 0.0 },
                    C1 => 1.0 / 16.0,
                    E3 => 1.0 / 16.0,
                    H2 => 1.0 / 16.0,
                    // Go to next R + Go to R1
                    R1 => match ch { CH3 => 3.0 / 16.0, _ => 1.0 / 16.0 },
                    // Go to next R only
                    R2 => match ch { CH1 => 2.0 / 16.0, _ => 0.0 },
                    // Go to next R only
                    R3 => match ch { CH2 => 2.0 / 16.0, _ => 0.0 },
                    // Go to next U
                    U1 => match ch { CH1 | CH3 => 1.0 / 16.0, _ => 0.0 },
                    // Go to next U
                    U2 => match ch { CH2 => 1.0 / 16.0, _ => 0.0 },
                    _  if dst == src => 6.0 / 16.0,
                    _  => 0.0
                }
            },
            _ => { if dst == src { 1.0 } else { 0.0 } }
        }
    });

    let trans_g2j = create_mat(NUM_SQUARE, NUM_SQUARE, |dst, src| {
        match Square::from_uint(src) {
            G2J => match Square::from_uint(dst) { JAIL => 1.0, _ => 0.0 },
            _ => { if src == dst { 1.0 } else { 0.0 } }
        }
    });

    let trans_all_g2j = create_mat(NUM_SQUARE, NUM_SQUARE, |dst, _src| {
        match Square::from_uint(dst) { JAIL => 1.0, _ => 0.0 }
    });

    let trans_square = mul_mat(mul_mat(trans_cc, trans_ch), trans_g2j);
    let trans_sq_singles  = mul_mat(trans_square, trans_singles);
    let trans_sq_doubles  = mul_mat(trans_square, trans_doubles);
    let trans_g2j_doubles = mul_mat(trans_all_g2j, trans_doubles);

    create_mat(NUM_SQUARE * 3, NUM_SQUARE * 3, |i, j| {
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
            _      => 0.0
        }
    })

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
    let mut vec = create_mat(trans.len(), 1, |i, _j| if i == 0 { 1.0 } else { 0.0 });

    loop {
        let vec2 = mul_mat(trans, vec);
        let sub = sub_mat(vec2, vec);
        let err = mul_mat(trans_mat(sub), sub);
        if err[0][0] < 1e-10 { break }
        vec = vec2;
    }

    let mut pairs = vec::from_fn(NUM_SQUARE, |i| (0.0, Square::from_uint(i)));
    for (i, vs) in vec.iter().enumerate() {
        let dst = i % NUM_SQUARE;
        let (p, sq) = pairs[dst];
        pairs[dst] = (p + vs[0], sq);
    }
    pairs.sort_by(|&(p1, _), &(p2, _)| {
            match () {
                _ if p2 <  p1 => Less,
                _ if p2 == p1 => Equal,
                _ if p2 >  p1 => Greater,
                _ => fail!()
            }
        });
    return pairs.slice(0, 3).map(|&(_, sq)| { format!("{:02}", sq.to_uint()) }).concat();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_int_convertible_square() {
        for n in range(0u, 40) {
            let sq = Square::from_uint(n);
            assert_eq!(sq.to_uint(), n);
        }
    }

    #[test]
    fn test_mul_mat() {
        assert_eq!(mul_mat([~[1.0, 2.0, 3.0]], [~[1.0], ~[2.0], ~[3.0]]),
                   ~[~[14.0]]);
        assert_eq!(mul_mat([~[1.0, 2.0, 3.0], ~[4.0, 5.0, 6.0], ~[7.0, 8.0, 9.0]],
                           [~[1.0], ~[0.0], ~[0.0]]),
                   ~[~[1.0], ~[4.0], ~[7.0]]);
        assert_eq!(mul_mat([~[1.0, 2.0, 3.0], ~[4.0, 5.0, 6.0], ~[7.0, 8.0, 9.0]],
                           [~[1.0, 0.0], ~[0.0, 0.0], ~[0.0, 1.0]]),
                   ~[~[1.0, 3.0], ~[4.0, 6.0], ~[7.0, 9.0]]);
    }
}
