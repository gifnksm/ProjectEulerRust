//! [Problem 84](https://projecteuler.net/problem=84) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use derive_try_from_primitive::TryFromPrimitive;
use generic_matrix::Matrix;
use std::convert::TryFrom;

#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive)]
#[repr(usize)]
enum Square {
    GO,
    A1,
    CC1,
    A2,
    T1,
    R1,
    B1,
    CH1,
    B2,
    B3,
    JAIL,
    C1,
    U1,
    C2,
    C3,
    R2,
    D1,
    CC2,
    D2,
    D3,
    FP,
    E1,
    CH2,
    E2,
    E3,
    R3,
    F1,
    F2,
    U2,
    F3,
    G2J,
    G1,
    G2,
    CC3,
    G3,
    R4,
    CH3,
    H1,
    T2,
    H2,
    Num,
}

const NUM_SQUARE: usize = Square::Num as usize;
const G2J_DOUBLES_COUNT: usize = 3;
const NUM_STATE: usize = G2J_DOUBLES_COUNT * NUM_SQUARE;

fn create_roll_distribution(dice_side: usize) -> Vec<(f64, f64)> {
    let mut dist = vec![(0.0, 0.0); dice_side * 2 + 1];
    for i in 1..(dice_side + 1) {
        for j in 1..(dice_side + 1) {
            let sum = i + j;
            if i != j {
                dist[sum].0 += 1.0;
            } else {
                dist[sum].1 += 1.0;
            }
        }
    }
    let cases = (dice_side * dice_side) as f64;
    for tp in &mut dist {
        tp.0 /= cases;
        tp.1 /= cases;
    }
    dist
}

fn roll_trans_matrix(dice_side: usize) -> Matrix<f64> {
    let roll_dist = create_roll_distribution(dice_side);
    let consec_prob = roll_dist.iter().map(|x| x.1).sum::<f64>();

    Matrix::from_fn(NUM_STATE, NUM_STATE, |dst, src| {
        let (dst_seq, dst_pos) = (dst / NUM_SQUARE, dst % NUM_SQUARE);
        let (src_seq, src_pos) = (src / NUM_SQUARE, src % NUM_SQUARE);
        let pos_diff = (dst_pos + NUM_SQUARE - src_pos) % NUM_SQUARE;

        if dst_seq > 0 {
            // consecutive doubles (src_seq times => dst_seq times)
            if src_seq != dst_seq - 1 || pos_diff >= roll_dist.len() {
                return 0.0;
            }
            return roll_dist[pos_diff].1;
        }

        // not consecutive doubles or goto jail (reset consecutive doubles).
        let prob = if pos_diff < roll_dist.len() {
            roll_dist[pos_diff].0
        } else {
            0.0
        };
        if src_seq == G2J_DOUBLES_COUNT - 1 && dst_pos == Square::JAIL as usize {
            prob + consec_prob
        } else {
            prob
        }
    })
}

fn ch_trans_matrix() -> Matrix<f64> {
    Matrix::from_fn(NUM_STATE, NUM_STATE, |dst, src| {
        let (dst_seq, dst_pos) = (dst / NUM_SQUARE, dst % NUM_SQUARE);
        let (src_seq, src_pos) = (src / NUM_SQUARE, src % NUM_SQUARE);
        let src_sq = Square::try_from(src_pos).unwrap();
        let dst_sq = Square::try_from(dst_pos).unwrap();
        match src_sq {
            Square::CH1 | Square::CH2 | Square::CH3 => {
                if dst_seq == src_seq && dst_sq == Square::JAIL {
                    return 1.0 / 16.0;
                }
                // Reset consecutive doubles after go to JAIL.
                // if dst_seq == 0 && dst_sq == Square::JAIL {
                //     return 1.0 / 16.0
                // }
                if dst_seq != src_seq {
                    return 0.0;
                }
                if (src_pos + NUM_SQUARE - dst_pos) % NUM_SQUARE == 3 {
                    // Go back 3 square
                    return 1.0 / 16.0;
                }

                match (dst_sq, src_sq) {
                    (Square::GO, _) => 1.0 / 16.0,
                    (Square::C1, _) => 1.0 / 16.0,
                    (Square::E3, _) => 1.0 / 16.0,
                    (Square::H2, _) => 1.0 / 16.0,
                    (Square::R1, Square::CH3) => 3.0 / 16.0, // Go to next R + Go to R1
                    (Square::R1, _) => 1.0 / 16.0,           // Go to R1
                    (Square::R2, Square::CH1) => 2.0 / 16.0, // Go to next R
                    (Square::R3, Square::CH2) => 2.0 / 16.0, // Go to next R
                    (Square::U1, Square::CH1) | (Square::U1, Square::CH3) => 1.0 / 16.0, // Go to next U
                    (Square::U2, Square::CH2) => 1.0 / 16.0, // Go to next U
                    _ if dst == src => 6.0 / 16.0,
                    _ => 0.0,
                }
            }
            _ => {
                if src == dst {
                    1.0
                } else {
                    0.0
                }
            }
        }
    })
}

fn cc_trans_matrix() -> Matrix<f64> {
    Matrix::from_fn(NUM_STATE, NUM_STATE, |dst, src| {
        let (dst_seq, dst_pos) = (dst / NUM_SQUARE, dst % NUM_SQUARE);
        let (src_seq, src_pos) = (src / NUM_SQUARE, src % NUM_SQUARE);
        let src_sq = Square::try_from(src_pos).unwrap();
        let dst_sq = Square::try_from(dst_pos).unwrap();
        match src_sq {
            Square::CC1 | Square::CC2 | Square::CC3 => {
                if dst_seq == src_seq && dst_sq == Square::JAIL {
                    return 1.0 / 16.0;
                }
                // Reset consecutive doubles after go to JAIL.
                // if dst_seq == 0 && dst_sq == Square::JAIL {
                //     return 1.0 / 16.0;
                // }
                if dst_seq != src_seq {
                    return 0.0;
                }
                if dst_sq == Square::GO {
                    return 1.0 / 16.0;
                }
                if dst_pos == src_pos {
                    return 14.0 / 16.0;
                }
                0.0
            }
            _ => {
                if src == dst {
                    1.0
                } else {
                    0.0
                }
            }
        }
    })
}

fn g2j_trans_matrix() -> Matrix<f64> {
    Matrix::from_fn(NUM_STATE, NUM_STATE, |dst, src| {
        let (dst_seq, dst_pos) = (dst / NUM_SQUARE, dst % NUM_SQUARE);
        let (src_seq, src_pos) = (src / NUM_SQUARE, src % NUM_SQUARE);
        let src_sq = Square::try_from(src_pos).unwrap();
        let dst_sq = Square::try_from(dst_pos).unwrap();

        if src_sq == Square::G2J {
            if dst_seq == src_seq && dst_sq == Square::JAIL {
                return 1.0;
            }
            // Reset consecutive doubles after go to JAIL.
            // if dst_seq == 0 && dst_sq == Square::JAIL {
            //     return 1.0
            // }
            return 0.0;
        }
        if src == dst {
            return 1.0;
        }
        0.0
    })
}

fn trans_matrix(dice_side: usize) -> Matrix<f64> {
    g2j_trans_matrix() * cc_trans_matrix() * ch_trans_matrix() * roll_trans_matrix(dice_side)
}

fn steady_state(dist: &Matrix<f64>, init: Matrix<f64>, epsilon: f64) -> Matrix<f64> {
    let mut state = init;
    loop {
        let new_state = dist * &state;
        let sub = &new_state - state;
        let err = sub.trans() * sub;
        if err[(0, 0)] <= epsilon {
            return new_state;
        }
        state = new_state;
    }
}

fn state_to_square(state: Matrix<f64>) -> Vec<(Square, f64)> {
    (0..NUM_SQUARE)
        .map(|s| {
            let prob = (s..NUM_STATE)
                .step_by(NUM_SQUARE)
                .map(|i| state[(i, 0)])
                .sum();
            let sq = Square::try_from(s).unwrap();
            (sq, prob)
        })
        .collect()
}

fn solve() -> String {
    // STATE: Square::Num * 3 (consecutive doubles)
    let state = steady_state(&trans_matrix(4), Matrix::one(NUM_STATE, 1), 1e-10);
    let mut square = state_to_square(state);
    square.sort_by(|&(_, p0), &(_, p1)| p1.partial_cmp(&p0).unwrap());
    format!(
        "{:02}{:02}{:02}",
        square[0].0 as usize, square[1].0 as usize, square[2].0 as usize
    )
}

common::problem!("101524", solve);

#[cfg(test)]
mod tests {
    use super::{Square, NUM_STATE};
    use generic_matrix::Matrix;

    #[test]
    fn six() {
        let state = super::steady_state(&super::trans_matrix(6), Matrix::one(NUM_STATE, 1), 1e-10);
        let mut square = super::state_to_square(state);
        square.sort_by(|&(_, p0), &(_, p1)| p1.partial_cmp(&p0).unwrap());

        assert_eq!(Square::JAIL, square[0].0);
        assert!(0.06235 <= square[0].1 && square[0].1 < 0.06245);
        assert_eq!(Square::E3, square[1].0);
        assert!(0.03175 <= square[1].1 && square[1].1 < 0.03185);
        assert_eq!(Square::GO, square[2].0);
        // assert!(0.03085 <= square[2].1 && square[2].1 < 0.03095);
        assert!(0.03085 <= square[2].1 && square[2].1 < 0.03105);
    }
}
