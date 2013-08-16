use std::char;

pub enum Suit {
    Dummy, Spade, Heart, Dia, Club
}

impl ToStr for Suit {
    fn to_str(&self) -> ~str {
        match self {
            &Dummy => ~"_",
            &Spade => ~"S",
            &Heart => ~"H",
            &Dia   => ~"D",
            &Club  => ~"C"
        }
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

pub struct Card {
    suit: Suit,
    num: uint
}

impl ToStr for Card {
    fn to_str(&self) -> ~str {
        let n = match self.num {
            10 => ~"T",
            11 => ~"J",
            12 => ~"Q",
            13 => ~"K",
            1  => ~"A",
            0  => ~"_",
            n  => n.to_str()
        };
        return fmt!("%s%s", n, self.suit.to_str());
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
        return suit.map(|s| Card { suit: *s, num: num} );
    }
}

impl Card {
    pub fn dummy() -> Card { Card { num: 0, suit: Dummy } }
}
