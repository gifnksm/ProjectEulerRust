//! [Problem 54](https://projecteuler.net/problem=54) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use playing_card::SuitCard as Card;
use std::{
    cmp::Ordering,
    fmt,
    fs::File,
    io::{self, prelude::*, BufReader},
    str::FromStr,
};

fn cmp_card(c0: &Card, c1: &Card) -> Ordering {
    if c0.num == c1.num {
        return Ordering::Equal;
    }
    if c0.num == 1 {
        return Ordering::Greater;
    }
    if c1.num == 1 {
        return Ordering::Less;
    }
    c0.num.cmp(&c1.num)
}

fn cmp_card_array(a0: &[Card], a1: &[Card]) -> Ordering {
    assert_eq!(a0.len(), a1.len());
    for (c0, c1) in a0.iter().zip(a1.iter()) {
        let ord = cmp_card(c0, c1);
        if ord != Ordering::Equal {
            return ord;
        }
    }
    Ordering::Equal
}

fn cmp_card_2darray(as0: &[&[Card]], as1: &[&[Card]]) -> Ordering {
    assert_eq!(as0.len(), as1.len());
    for (&a0, &a1) in as0.iter().zip(as1.iter()) {
        let ord = cmp_card_array(a0, a1);
        if ord != Ordering::Equal {
            return ord;
        }
    }
    Ordering::Equal
}

fn sort_cards(cs: &mut [Card]) {
    cs.sort_by(|c0, c1| match cmp_card(c0, c1) {
        Ordering::Equal => (c0.suit as u32).cmp(&(c1.suit as u32)),
        Ordering::Less => Ordering::Greater,
        Ordering::Greater => Ordering::Less,
    })
}

type C1 = [Card; 1];
type C2 = [Card; 2];
type C3 = [Card; 3];
type C4 = [Card; 4];
type C5 = [Card; 5];

#[derive(Eq, Debug)]
enum Hand {
    HighCard(C1, C1, C1, C1, C1),
    Pair(C2, C1, C1, C1),
    TwoPairs(C2, C2, C1),
    ThreeOfAKind(C3, C1, C1),
    Straight(C5),
    Flush(C5),
    FullHouse(C3, C2),
    FourOfAKind(C4, C1),
    StraightFlush(C5),
    RoyalFlush(C5),
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Hand::HighCard([c0], [c1], [c2], [c3], [c4]) => {
                write!(f, "HighCard({}, {}, {}, {}, {})", c0, c1, c2, c3, c4)
            }
            Hand::Pair([c0, c1], [c2], [c3], [c4]) => {
                write!(f, "Pair({}, {}) + HighCard({}, {}, {})", c0, c1, c2, c3, c4)
            }
            Hand::TwoPairs([c0, c1], [c2, c3], [c4]) => write!(
                f,
                "TwoPairs(({}, {}), ({}, {})) + HighCard({})",
                c0, c1, c2, c3, c4
            ),
            Hand::ThreeOfAKind([c0, c1, c2], [c3], [c4]) => write!(
                f,
                "ThreeOfAKind({}, {}, {}) + HighCard({}, {})",
                c0, c1, c2, c3, c4
            ),
            Hand::Straight([c0, c1, c2, c3, c4]) => {
                write!(f, "Straight({}, {}, {}, {}, {})", c0, c1, c2, c3, c4)
            }
            Hand::Flush([c0, c1, c2, c3, c4]) => {
                write!(f, "Flush({}, {}, {}, {}, {})", c0, c1, c2, c3, c4)
            }
            Hand::FullHouse([c0, c1, c2], [c3, c4]) => {
                write!(f, "FullHouse(({}, {}, {}), ({}, {}))", c0, c1, c2, c3, c4)
            }
            Hand::FourOfAKind([c0, c1, c2, c3], [c4]) => write!(
                f,
                "FourOfAKind({}, {}, {}, {}) + HighCard({})",
                c0, c1, c2, c3, c4
            ),
            Hand::StraightFlush([c0, c1, c2, c3, c4]) => {
                write!(f, "StraightFlush({}, {}, {}, {}, {})", c0, c1, c2, c3, c4)
            }
            Hand::RoyalFlush([c0, c1, c2, c3, c4]) => {
                write!(f, "RoyalFlush({}, {}, {}, {}, {})", c0, c1, c2, c3, c4)
            }
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Hand) -> bool {
        self.to_array() == other.to_array()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        match self.rank().cmp(&other.rank()) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => cmp_card_2darray(&self.to_vec_of_array(), &other.to_vec_of_array()),
        }
    }
}

impl Hand {
    fn pair(mut p0: C2, s0: Card, s1: Card, s2: Card) -> Hand {
        sort_cards(&mut p0);
        let mut ss = [s0, s1, s2];
        sort_cards(&mut ss);
        Hand::Pair(p0, [ss[0]], [ss[1]], [ss[2]])
    }

    fn two_pair(mut p0: C2, mut p1: C2, s0: Card) -> Hand {
        sort_cards(&mut p0);
        sort_cards(&mut p1);
        match cmp_card(&p0[0], &p1[0]) {
            Ordering::Less => Hand::TwoPairs(p1, p0, [s0]),
            Ordering::Greater => Hand::TwoPairs(p0, p1, [s0]),
            Ordering::Equal => panic!(),
        }
    }

    fn three_of_a_kind(mut t0: C3, s0: Card, s1: Card) -> Hand {
        sort_cards(&mut t0);
        let mut ss = [s0, s1];
        sort_cards(&mut ss);
        Hand::ThreeOfAKind(t0, [ss[0]], [ss[1]])
    }

    fn full_house(mut t0: C3, mut p0: C2) -> Hand {
        sort_cards(&mut t0);
        sort_cards(&mut p0);
        Hand::FullHouse(t0, p0)
    }

    fn four_of_a_kind(mut q0: C4, s0: Card) -> Hand {
        sort_cards(&mut q0);
        Hand::FourOfAKind(q0, [s0])
    }

    fn from_cards(cards: &[Card]) -> Hand {
        assert_eq!(5, cards.len());

        let mut num_count = (0..13).map(|_| vec![]).collect::<Vec<_>>();
        let mut suit_count = (0..4).map(|_| vec![]).collect::<Vec<_>>();

        for &c in cards {
            let val = if c.num == 1 { 12 } else { c.num - 2 };
            num_count[(12 - val) as usize].push(c);
            suit_count[c.suit as usize].push(c);
        }

        let num_count = num_count;
        let suit_count = suit_count;

        let mut single = vec![];
        let mut pairs = vec![];
        let mut three = vec![];
        let mut four = vec![];
        for v in &num_count {
            match v.len() {
                0 => {
                    // Do nothing
                }
                1 => single.push(v[0]),
                2 => pairs.push([v[0], v[1]]),
                3 => three.push([v[0], v[1], v[2]]),
                4 => four.push([v[0], v[1], v[2], v[3]]),
                _ => panic!(),
            }
        }

        assert_eq!(
            5,
            single.len() + pairs.len() * 2 + three.len() * 3 + four.len() * 4
        );

        match (pairs.len(), three.len(), four.len()) {
            (1, 0, 0) => return Hand::pair(pairs[0], single[0], single[1], single[2]),
            (2, 0, 0) => return Hand::two_pair(pairs[0], pairs[1], single[0]),
            (0, 1, 0) => return Hand::three_of_a_kind(three[0], single[0], single[1]),
            (1, 1, 0) => return Hand::full_house(three[0], pairs[0]),
            (0, 0, 1) => return Hand::four_of_a_kind(four[0], single[0]),
            _ => {
                // Do nothing
            }
        }

        let is_flush = suit_count.iter().any(|v| v.len() == 5);
        let mut is_straight = {
            let min_idx = num_count.iter().position(|v| !v.is_empty()).unwrap();
            num_count[min_idx..(min_idx + 5)]
                .iter()
                .all(|v| v.len() == 1)
        };

        let mut ss = [single[0], single[1], single[2], single[3], single[4]];
        sort_cards(&mut ss);
        if ss[0].num == 1 && ss[1].num == 5 && ss[2].num == 4 && ss[3].num == 3 && ss[4].num == 2 {
            ss = [ss[1], ss[2], ss[3], ss[4], ss[0]];
            is_straight = true;
        }

        match (is_flush, is_straight) {
            (true, true) if ss[0].num == 1 => Hand::RoyalFlush(ss),
            (true, true) => Hand::StraightFlush(ss),
            (true, false) => Hand::Flush(ss),
            (false, true) => Hand::Straight(ss),
            (false, false) => Hand::HighCard([ss[0]], [ss[1]], [ss[2]], [ss[3]], [ss[4]]),
        }
    }

    fn rank(&self) -> u32 {
        match *self {
            Hand::HighCard(..) => 0,
            Hand::Pair(..) => 1,
            Hand::TwoPairs(..) => 2,
            Hand::ThreeOfAKind(..) => 3,
            Hand::Straight(..) => 4,
            Hand::Flush(..) => 5,
            Hand::FullHouse(..) => 6,
            Hand::FourOfAKind(..) => 7,
            Hand::StraightFlush(..) => 8,
            Hand::RoyalFlush(..) => 9,
        }
    }

    fn to_array(&self) -> C5 {
        match *self {
            Hand::HighCard([c0], [c1], [c2], [c3], [c4])
            | Hand::Pair([c0, c1], [c2], [c3], [c4])
            | Hand::TwoPairs([c0, c1], [c2, c3], [c4])
            | Hand::ThreeOfAKind([c0, c1, c2], [c3], [c4])
            | Hand::Straight([c0, c1, c2, c3, c4])
            | Hand::Flush([c0, c1, c2, c3, c4])
            | Hand::FullHouse([c0, c1, c2], [c3, c4])
            | Hand::FourOfAKind([c0, c1, c2, c3], [c4])
            | Hand::StraightFlush([c0, c1, c2, c3, c4])
            | Hand::RoyalFlush([c0, c1, c2, c3, c4]) => [c0, c1, c2, c3, c4],
        }
    }

    fn to_vec_of_array<'a>(&'a self) -> Vec<&'a [Card]> {
        match *self {
            Hand::HighCard(ref s0, ref s1, ref s2, ref s3, ref s4) => vec![s0, s1, s2, s3, s4],
            Hand::Pair(ref p0, ref s0, ref s1, ref s2) => vec![p0, s0, s1, s2],
            Hand::TwoPairs(ref p0, ref p1, ref s0) => vec![p0, p1, s0],
            Hand::ThreeOfAKind(ref t0, ref s0, ref s1) => vec![t0, s0, s1],
            Hand::Straight(ref cs) => vec![cs],
            Hand::Flush(ref cs) => vec![cs],
            Hand::FullHouse(ref t0, ref p0) => vec![t0, p0],
            Hand::FourOfAKind(ref q0, ref s0) => vec![q0, s0],
            Hand::StraightFlush(ref cs) => vec![cs],
            Hand::RoyalFlush(ref cs) => vec![cs],
        }
    }
}

fn solve(file: File) -> io::Result<String> {
    let mut p1_win = 0;
    let mut _p2_win = 0;
    let mut _draw = 0;
    for line in BufReader::new(file).lines() {
        let line = line?;
        let cards = line
            .trim()
            .split(' ')
            .map(|c| FromStr::from_str(c).unwrap())
            .collect::<Vec<_>>();
        let p1_hand = Hand::from_cards(&cards[..5]);
        let p2_hand = Hand::from_cards(&cards[5..]);
        match p1_hand.cmp(&p2_hand) {
            Ordering::Greater => p1_win += 1,
            Ordering::Less => _p2_win += 1,
            Ordering::Equal => _draw += 1,
        }
    }

    Ok(p1_win.to_string())
}

common::problem!("376", "p054_poker.txt", solve);

#[cfg(test)]
mod tests {
    use super::Hand;
    use playing_card::SuitCard as Card;
    use rand::{self, seq::SliceRandom};
    use std::{cmp::Ordering, str::FromStr};

    fn str_to_cards(s: &str) -> Vec<Card> {
        s.split(' ')
            .map(|c| FromStr::from_str(c).unwrap())
            .collect::<Vec<_>>()
    }

    #[test]
    fn from_cards() {
        fn check(input: &str, output: &str) {
            let mut cs = str_to_cards(input);
            let ihand = Hand::from_cards(&cs);
            assert_eq!(output, &ihand.to_string()[..]);

            let mut rng = rand::thread_rng();
            for _ in 0..10 {
                cs.shuffle(&mut rng);
                let hand = Hand::from_cards(&cs);
                assert_eq!(ihand, hand);
                assert_eq!(output, &hand.to_string()[..]);
            }
        }

        check("AC JS 9S 8C 5D", "HighCard(AC, JS, 9S, 8C, 5D)");
        check("QH 8S 7D 5C 2C", "HighCard(QH, 8S, 7D, 5C, 2C)");

        check("3D 3C AD JS 4H", "Pair(3D, 3C) + HighCard(AD, JS, 4H)");
        check("5H 5C KD 7S 6S", "Pair(5H, 5C) + HighCard(KD, 7S, 6S)");
        check("QH QC 9H 6S 4D", "Pair(QH, QC) + HighCard(9H, 6S, 4D)");
        check("QS QD 7H 6D 3D", "Pair(QS, QD) + HighCard(7H, 6D, 3D)");

        check(
            "QH QC 2H 2D 7C",
            "TwoPairs((QH, QC), (2H, 2D)) + HighCard(7C)",
        );

        check(
            "9S 9H 9D QS 7C",
            "ThreeOfAKind(9S, 9H, 9D) + HighCard(QS, 7C)",
        );

        check("9C 8H 7D 6D 5S", "Straight(9C, 8H, 7D, 6D, 5S)");

        check("KC TC 8C 5C 2C", "Flush(KC, TC, 8C, 5C, 2C)");

        check("6S 6H 6C 3D 3C", "FullHouse((6S, 6H, 6C), (3D, 3C))");
        check("4S 4D 4C 2H 2D", "FullHouse((4S, 4D, 4C), (2H, 2D))");
        check("3S 3D 3C 9S 9D", "FullHouse((3S, 3D, 3C), (9S, 9D))");

        check(
            "8S 8H 8D 8C 2D",
            "FourOfAKind(8S, 8H, 8D, 8C) + HighCard(2D)",
        );
        check("7H 6H 5H 4H 3H", "StraightFlush(7H, 6H, 5H, 4H, 3H)");

        check("AS KS QS JS TS", "RoyalFlush(AS, KS, QS, JS, TS)");
    }

    #[test]
    fn cmp() {
        fn check(order: Ordering, left: &str, right: &str) {
            let lh = Hand::from_cards(&str_to_cards(left));
            let rh = Hand::from_cards(&str_to_cards(right));
            assert_eq!(order, lh.cmp(&rh));
            assert_eq!(order.reverse(), rh.cmp(&lh));
        }

        check(Ordering::Greater, "5D 8C 9S JS AC", "2C 5C 7D 8S QH");
        check(Ordering::Greater, "AD TD 9S 5C 4C", "KS QD JC 8H 7H");
        check(Ordering::Greater, "AC QC 7D 5H 2C", "AD TD 9S 5C 4C");

        check(Ordering::Less, "5H 5C 6S 7S KD", "2C 3S 8S 8D TD");
        check(Ordering::Greater, "4D 6S 9H QH QC", "3D 6D 7H QD QS");
        check(Ordering::Greater, "TC TS 6S 4H 2H", "9H 9C AH QD TD");
        check(Ordering::Greater, "2D 2H 8S 5C 4C", "2C 2S 8C 5H 3H");

        check(Ordering::Greater, "KH KD 2C 2D JH", "JD JS TS TC 9S");
        check(Ordering::Greater, "9C 9D 7D 7S 6H", "9H 9S 5H 5D KC");
        check(Ordering::Greater, "4S 4C 3S 3H KD", "4H 4D 3D 3C TS");

        check(Ordering::Greater, "QS QC QD 5S 3C", "5C 5H 5D QD TC");
        check(Ordering::Greater, "8C 8H 8C AC 2D", "8S 8H 8D 5S 3C");

        check(Ordering::Greater, "8S 7S 6H 5H 4S", "6D 5S 4D 3H 2C");
        check(Ordering::Equal, "8S 7S 6H 5H 4S", "8H 7D 6C 5C 4H");

        check(Ordering::Greater, "AH QH TH 5H 3H", "KS QS JS 9S 6S");
        check(Ordering::Greater, "AD KD 7D 6D 2D", "AH QH TH 5H 3H");

        check(Ordering::Greater, "2H 2D 4C 4D 4S", "3C 3D 3S 9S 9D");
        check(Ordering::Greater, "TS TH TD 4S 4D", "9H 9C 9S AH AC");
        check(Ordering::Greater, "AS AC AH 4D 4C", "AS AH AD 3S 3D");

        check(Ordering::Greater, "TC TD TH TS 5D", "6D 6H 6S 6C KS");
        check(Ordering::Greater, "TC TD TH TS QC", "TC TD TH TS 5D");

        check(Ordering::Less, "7C 6C 5C 4C 3C", "AH KH QH JH TH");
        check(Ordering::Greater, "7H 6H 5H 4H 3H", "5S 4S 3S 2S 1S");
        check(Ordering::Equal, "JC TC 9C 8C 7C", "JD TD 9D 8D 7D");

        check(Ordering::Less, "2D 9C AS AH AC", "3D 6D 7D TD QD");
        check(Ordering::Greater, "5S 4S 3S 2S 1S", "TC TH TD TS 3H");
    }
}
