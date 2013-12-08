#[link(name = "prob0054", vers = "0.0", package_id = "prob0054")];
#[crate_type = "lib"];

extern mod common;
extern mod data;

use std::{iter, vec};
use std::io::File;
use std::io::buffered::BufferedReader;
use common::reader::BufferedReaderUtil;
use data::card::Card;

pub static EXPECTED_ANSWER: &'static str = "376";

enum Hand {
    Hi            ( [Card, ..5] ),
    Pair          ( [Card, ..2],        [Card, ..3] ),
    TwoPair       ( [[Card, ..2], ..2], [Card, ..1] ),
    Three         ( [Card, ..3],        [Card, ..2] ),
    Straight      ( [Card, ..5] ),
    Flush         ( [Card, ..5] ),
    FullHouse     ( ([Card, ..3], [Card, ..2]) ),
    Four          ( [Card, ..4],        [Card, ..1] ),
    StraightFlush ( [Card, ..5] ),
}

impl ToStr for Hand {
    fn to_str(&self) -> ~str {
        match self {
            &Hi(ref c) => {
                format!("Hi({})", c.to_str())
            },
            &Pair(ref c, ref r) => {
                format!("Pair({}) + Hi({})", c.to_str(), r.to_str())
            },
            &TwoPair(ref c, ref r) => {
                format!("TwoPair({}, {}) + Hi({})",
                     c[0].to_str(), c[1].to_str(), r.to_str())
            },
            &Three(ref c, ref r) => {
                format!("Three({}) + Hi({})", c.to_str(), r.to_str())
            },
            &Straight(ref c) => {
                format!("Straight({})", c.to_str())
            },
            &Flush(ref c) => {
                format!("Flush({})", c.to_str())
            },
            &FullHouse((ref c3, ref c2)) => {
                format!("FullHouse({}, {})", c3.to_str(), c2.to_str())
            },
            &Four(ref c, ref r) => {
                format!("Four({}) + Hi({})", c.to_str(), r.to_str())
            },
            &StraightFlush(ref c) => {
                format!("StraightFlush({})", c.to_str())
            },
        }
    }
}

fn hand(cards: &[Card, ..5]) -> Hand {
    let mut num_count = vec::from_fn(13, |_i| ~[]);
    let mut suit_count = vec::from_fn(4, |_i| ~[]);

    for &c in cards.iter() {
        let val = if c.num == 1 { 12 } else { c.num - 2 };
        num_count[12 - val].push(c);
        suit_count[c.suit as uint - 1].push(c);
    }

    let num_count = num_count;
    let suit_count = suit_count;

    let mut single = ~[];
    let mut pairs = ~[];
    let mut three = ~[];
    let mut four  = ~[];
    for v in num_count.iter() {
        match v.len() {
            0 => { /* Do nothing */ },
            1 => single.push(v[0]),
            2 => pairs.push([v[0], v[1]]),
            3 => three.push([v[0], v[1], v[2]]),
            4 => four.push([v[0], v[1], v[2], v[3]]),
            _ => fail!()
        }
    }
    match (pairs.len(), three.len(), four.len()) {
        (1, 0, 0) => return Pair      (pairs[0],
                                       [ single[0], single[1], single[2] ]),
        (2, 0, 0) => return TwoPair   ([ pairs[0], pairs[1] ],
                                       [ single[0] ]),
        (0, 1, 0) => return Three     (three[0],
                                       [ single[0], single[1] ]),
        (1, 1, 0) => return FullHouse ((three[0], pairs[0])),
        (0, 0, 1) => return Four      (four[0],
                                       [ single[0] ]),
        _ => { /* Do nothing */ }
    }

    let is_flush = suit_count.iter().any(|v| v.len() == 5);
    let is_straight = {
        let mut min_idx = 0;
        for (i, v) in num_count.iter().enumerate() {
            if v.len() > 0 {
                min_idx = i;
                break;
            }
        }
        num_count.slice(min_idx, min_idx + 5).iter().all(|v| v.len() == 1)
    };

    return match (is_flush, is_straight) {
        (true, true) => StraightFlush([ single[0], single[1], single[2], single[3], single[4] ]),
        (true, false) => Flush([ single[0], single[1], single[2], single[3], single[4] ]),
        (false, true) => Straight([ single[0], single[1], single[2], single[3], single[4] ]),
        (false, false) => Hi([ single[0], single[1], single[2], single[3], single[4] ])
    }
}

#[inline(always)]
fn cmp_card(c1: &Card, c2: &Card) -> int {
    if c1.num == c2.num { return 0; }
    if c1.num == 1      { return  1; }
    if c2.num == 1      { return -1; }
    return (c1.num as int) - (c2.num as int);
}

#[inline(always)]
fn cmp_cards(cs1: &[Card], cs2: &[Card]) -> int {
    assert_eq!(cs1.len(), cs2.len());
    for (c1, c2) in  cs1.iter().zip(cs2.iter()) {
        let cmp = cmp_card(c1, c2);
        if cmp != 0 { return cmp; }
    }
    return 0;
}

#[inline(always)]
fn cmp_cards2(cs1_hi: &[Card], cs1_lo: &[Card],
              cs2_hi: &[Card], cs2_lo: &[Card]) -> int {
    let cmp = cmp_cards(cs1_hi, cs2_hi);
    if cmp != 0 { return cmp; }
    return cmp_cards(cs1_lo, cs2_lo);
}

#[inline(always)]
fn cmp_cards3(cs1_hi: &[Card], cs1_mid: &[Card], cs1_lo: &[Card],
              cs2_hi: &[Card], cs2_mid: &[Card], cs2_lo: &[Card]) -> int {
    let cmp = cmp_cards2(cs1_hi, cs1_mid, cs2_hi, cs2_mid);
    if cmp != 0 { return cmp; }
    return cmp_cards(cs1_lo, cs2_lo);
}

#[inline(always)]
fn ord_tuple<'a>(c1: &'a [Card], c2: &'a [Card]) -> (&'a [Card], &'a [Card]) {
    if cmp_cards(c1, c2) >= 0 { return (c1, c2); }
    return (c2, c1);
}

fn judge(p1_cards: &[Card, ..5], p2_cards: &[Card, ..5]) -> int {
    match (hand(p1_cards), hand(p2_cards)) {
        (StraightFlush(c1), StraightFlush(c2)) => { return cmp_cards(c1, c2); },
        (StraightFlush(..), _) => { return  1; },
        (_, StraightFlush(..)) => { return -1; },

        (Four(c1, r1), Four(c2, r2)) => {
            return cmp_cards2(c1, r1, c2, r2);
        },
        (Four(..), _) => { return  1; },
        (_, Four(..)) => { return -1; },

        (FullHouse((c1a, c1b)), FullHouse((c2a, c2b))) => {
            let (c1_hi, c1_lo) = ord_tuple(c1a, c1b);
            let (c2_hi, c2_lo) = ord_tuple(c2a, c2b);
            return cmp_cards2(c1_hi, c1_lo, c2_hi, c2_lo);
        }
        (FullHouse(..), _) => { return  1; },
        (_, FullHouse(..)) => { return -1; },

        (Flush(c1), Flush(c2)) => { return cmp_cards(c1, c2); },
        (Flush(..), _) => { return  1; },
        (_, Flush(..)) => { return -1; },

        (Straight(c1), Straight(c2)) => { return cmp_cards(c1, c2); },
        (Straight(..), _) => { return  1; },
        (_, Straight(..)) => { return -1; },

        (Three(c1, r1), Three(c2, r2)) => {
            return cmp_cards2(c1, r1, c2, r2);
        },
        (Three(..), _) => { return  1; },
        (_, Three(..)) => { return -1; },

        (TwoPair([c1_hi, c1_lo], r1), TwoPair([c2_hi, c2_lo], r2)) => {
            return cmp_cards3(c1_hi, c1_lo, r1, c2_hi, c2_lo, r2);
        },
        (TwoPair(..), _) => { return  1; },
        (_, TwoPair(..)) => { return -1; },

        (Pair(c1, r1), Pair(c2, r2)) => {
            return cmp_cards2(c1, r1, c2, r2);
        },
        (Pair(..), _) => { return  1; },
        (_, Pair(..)) => { return -1; },

        (Hi(c1), Hi(c2)) => { return cmp_cards(c1, c2); }
    };
}

pub fn solve() -> ~str {
    let r = File::open(&Path::new("files/poker.txt")).expect("file not found.");
    let mut br = BufferedReader::new(r);

    let mut p1_win = 0u;
    let mut _p2_win = 0u;
    let mut _draw = 0u;
    for line in br.line_iter() {
        let mut p1_cards = [ Card::dummy(), ..5 ];
        let mut p2_cards = [ Card::dummy(), ..5 ];
        for (word, i) in line.words().zip(iter::count(0u, 1)) {
            let cards = if i < 5 { &mut p1_cards } else { &mut p2_cards };
            cards[i % 5] = FromStr::from_str(word).unwrap();
        }
        let cmp = judge(&p1_cards, &p2_cards);
        if cmp > 0 { p1_win += 1;  }
        if cmp < 0 { _p2_win += 1;  }
        if cmp == 0 { _draw += 1;  }
    }
    p1_win.to_str()
}
