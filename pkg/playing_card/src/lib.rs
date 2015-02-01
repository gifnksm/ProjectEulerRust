//! Data types that represent playing cards.

#![warn(bad_style, missing_docs,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(core, collections, unicode)]

use std::fmt;
use std::str::FromStr;

use Suit::{Spade, Heart, Dia, Club};

/// Playing card's suite.
#[allow(missing_docs, unused_qualifications)] // FIXME rust-lang/rust#19102
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Suit {
    Spade,
    Heart,
    Dia,
    Club
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            &Spade => "S",
            &Heart => "H",
            &Dia   => "D",
            &Club  => "C"
        };

        write!(f, "{}", s)
    }
}

impl FromStr for Suit {
    fn from_str(s: &str) -> Option<Suit> {
        if s.len() != 1 { return None; }
        return match s {
            "S" => Some(Spade),
            "H" => Some(Heart),
            "D" => Some(Dia),
            "C" => Some(Club),
            _   => None
        };
    }
}

/// Playing card that only contains suit cards.
#[allow(missing_docs)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct SuitCard {
    pub num: u8,
    pub suit: Suit
}

impl fmt::Display for SuitCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SuitCard { num: 1,  suit: s } => write!(f, "A{}", s),
            SuitCard { num: 10, suit: s } => write!(f, "T{}", s),
            SuitCard { num: 11, suit: s } => write!(f, "J{}", s),
            SuitCard { num: 12, suit: s } => write!(f, "Q{}", s),
            SuitCard { num: 13, suit: s } => write!(f, "K{}", s),
            SuitCard { num: n,  suit: s } => write!(f, "{}{}", n, s)
        }
    }
}

impl FromStr for SuitCard {
    fn from_str(s: &str) -> Option<SuitCard> {
        if s.len() != 2 { return None }
        let (c0, c1) = s.slice_shift_char().unwrap();
        let suit = FromStr::from_str(c1);
                let num = match c0 {
                    'A' => Some(1),
                    'T' => Some(10),
                    'J' => Some(11),
                    'Q' => Some(12),
                    'K' => Some(13),
                    d => d.to_digit(10).map(|x| x as u8)
                };
        if let (Some(n), Some(s)) = (num, suit) {
            Some(SuitCard { num: n, suit: s })
        } else {
            None
        }
    }
}

/// Playing card that also contaiins jokers.
#[allow(missing_docs)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Card {
    Suit(SuitCard),
    BlackJoker,
    WhiteJoker
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Card::BlackJoker => write!(f, "BJ"),
            Card::WhiteJoker => write!(f, "WJ"),
            Card::Suit(sc) => write!(f, "{}", sc),
        }
    }
}

impl FromStr for Card {
    fn from_str(s: &str) -> Option<Card> {
        match s {
            "BJ" => Some(Card::BlackJoker),
            "WJ" => Some(Card::WhiteJoker),
            _    => FromStr::from_str(s).map(|sc| Card::Suit(sc))
        }
    }
}

impl Card {
    /// Creates new `SuitCard`.
    pub fn new(n: u8, s: Suit) -> Card {
        Card::Suit(SuitCard { num: n, suit: s })
    }
}

#[cfg(test)]
mod tests {
    use super::{Suit, Card};
    use super::Suit::{Spade, Heart, Dia, Club};

    #[test]
    fn show_suit() {
        fn check_pair(s: String, suite: Suit) {
            assert_eq!(s, format!("{}", suite));
            assert_eq!(Some(suite), s.parse());
        }
        check_pair("S".to_string(), Spade);
        check_pair("H".to_string(), Heart);
        check_pair("D".to_string(), Dia);
        check_pair("C".to_string(), Club);
    }

    #[test]
    fn show_card() {
        fn check_pair(s: String, card: Card) {
            assert_eq!(s, format!("{}", card));
            assert_eq!(Some(card), s.parse());
        }
        check_pair("BJ".to_string(), Card::BlackJoker);
        check_pair("WJ".to_string(), Card::WhiteJoker);
        check_pair("AH".to_string(), Card::new(1, Heart));
        check_pair("2C".to_string(), Card::new(2, Club));
        check_pair("9D".to_string(), Card::new(9, Dia));
        check_pair("TS".to_string(), Card::new(10, Spade));
        check_pair("JH".to_string(), Card::new(11, Heart));
        check_pair("QC".to_string(), Card::new(12, Club));
        check_pair("KD".to_string(), Card::new(13, Dia));
    }
}
