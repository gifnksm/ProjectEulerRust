//! [Problem 121](https://projecteuler.net/problem=121) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use num_bigint::BigUint;
use num_integer::Integer;
use num_rational::Ratio;
use num_traits::{FromPrimitive, One, Zero};
use polynomial::Polynomial;

// turn  blue    red
// 1     1/2     1/2
// 2     1/3     2/3
// 3     1/4     3/4
// 4     1/5     4/5
// .
// k     1/(k+1) k/(k+1)
//
// player wins:
//   blue: 4 times =>  1/2*1/3*1/4*1/5 = 1/120
//   blue: 3 times =>  1/2*1/3*1/4*4/5
//                   + 1/2*1/3*3/4*1/5
//                   + 1/2*2/3*1/4*1/5
//                   + 1/2*1/3*1/4*1/5 = 10/120

// (b + r)(b + 2r)(b + 3r)(b + 4r) / (2 * 3 * 4 * 5)
// = (b^4 + 10b^3r + 35b^2r^2 + 50br^3 + 24r^4) / (2 * 3 * 4 * 5)
// b := x, r := 1
// => (x^4 + 10x^3 + 35x^2 + 50x + 24) / (2 * 3 * 4 * 5)

fn probability_of_player_win<T: Integer + Clone + FromPrimitive>(turns: usize) -> Ratio<T> {
    (1..(turns + 1))
        .map(|t| T::from_usize(t).unwrap())
        .map(|t| {
            let denom = t.clone() + One::one();
            let blue = Ratio::new(One::one(), denom.clone());
            let red = Ratio::new(t, denom);
            Polynomial::new(vec![blue, red])
        })
        .fold(Polynomial::<Ratio<T>>::one(), |acc, elt| acc * elt)
        .data()
        .iter()
        .take((turns + 1) / 2)
        .fold(Ratio::<T>::zero(), |acc, elt| acc + elt)
}

fn max_prize<T: Integer + Clone>(p: Ratio<T>) -> T {
    p.denom().div_floor(p.numer())
}

fn solve() -> String {
    let prob = probability_of_player_win::<BigUint>(15);
    max_prize(prob).to_string()
}

common::problem!("2269", solve);

#[cfg(test)]
mod tests {
    use num_rational::Ratio;

    #[test]
    fn probability_of_player_win() {
        assert_eq!(Ratio::new(11, 120), super::probability_of_player_win(4));
    }

    #[test]
    fn max_prize() {
        assert_eq!(10, super::max_prize(Ratio::new(11, 120)));
    }
}
