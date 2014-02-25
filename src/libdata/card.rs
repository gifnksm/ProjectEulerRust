use std::{char, fmt};
use std::from_str::FromStr;

#[deriving(Eq)]
pub enum Suit {
    Dummy, Spade, Heart, Dia, Club
}

impl fmt::Show for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            &Dummy => "_",
            &Spade => "S",
            &Heart => "H",
            &Dia   => "D",
            &Club  => "C"
        };

        write!(f.buf, "{}", s)
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

#[deriving(Eq)]
pub struct Card {
    num: uint,
    suit: Suit
}

impl fmt::Show for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = match self.num {
            10 => ~"T",
            11 => ~"J",
            12 => ~"Q",
            13 => ~"K",
            1  => ~"A",
            0  => ~"_",
            n  => n.to_str()
        };

        write!(f.buf, "{}{}", n, self.suit)
    }
}

impl FromStr for Card {
    fn from_str(s: &str) -> Option<Card> {
        if s.len() != 2 { return None; }
        let suit = FromStr::from_str(s.slice(1, 2));
        let num = match s.char_at(0) {
            'A' => 1,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            d if char::is_digit(d) => char::to_digit(d, 10).unwrap(),
            _   => return None
        };
        return suit.map(|s| Card { suit: s, num: num} );
    }
}

impl Card {
    pub fn new(num: uint, suit: Suit) -> Card { Card { num: num, suit: suit } }
    pub fn dummy() -> Card { Card::new(0, Dummy) }
}

#[cfg(test)]
mod tests {
    use super::{Suit, Dummy, Spade, Heart, Dia, Club, Card};

    #[test]
    fn show_suit() {
        assert_eq!(~"_", format!("{}", Dummy));

        fn check_pair(s: ~str, suite: Suit) {
            assert_eq!(s, format!("{}", suite));
            assert_eq!(Some(suite), from_str(s));
        }
        check_pair(~"S", Spade);
        check_pair(~"H", Heart);
        check_pair(~"D", Dia);
        check_pair(~"C", Club);
    }

    #[test]
    fn show_card() {
        assert_eq!(~"__", format!("{}", Card::dummy()));

        fn check_pair(s: ~str, card: Card) {
            assert_eq!(s, format!("{}", card));
            assert_eq!(Some(card), from_str(s));
        }
        check_pair(~"AH", Card::new(1, Heart));
        check_pair(~"2C", Card::new(2, Club));
        check_pair(~"9D", Card::new(9, Dia));
        check_pair(~"TS", Card::new(10, Spade));
        check_pair(~"JH", Card::new(11, Heart));
        check_pair(~"QC", Card::new(12, Club));
        check_pair(~"KD", Card::new(13, Dia));
    }
}
