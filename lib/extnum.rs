use extcmp::{ Cmp, Eq, Lt, Gt, ExtOrd };
use cmp::{ Cmp, Ord };

enum Sign {
    Minus, Zero, Plus
}

impl Sign: ExtOrd {
    pure fn cmp(&&other: Sign) -> Cmp {
        match (self, other) {
          (Minus, Minus) | (Zero,  Zero) | (Plus, Plus) => Eq,
          (Minus, Zero)  | (Minus, Plus) | (Zero, Plus) => Lt,
          _                                             => Gt
        }
    }
}

impl Sign {
    pure fn neg() -> Sign {
        match(self) {
          Minus => Plus,
          Zero  => Zero,
          Plus  => Minus
        }
    }

    pure fn to_int() -> int {
        match self {
          Minus => -1,
          Zero  => 0,
          Plus  => 1
        }
    }

    static pure fn from_int(n: int) -> Sign {
        if n < 0  { return Minus; }
        if n == 0 { return Zero; }
        return Plus;
    }
}

impl Sign : Eq {
    pure fn eq(&&other: Sign) -> bool { match self.cmp(other) { Eq => true, _ => false } }
    pure fn ne(&&other: Sign) -> bool { !self.eq(other) }
}

impl Sign : Ord {
    pure fn lt(&&other: Sign) -> bool { match self.cmp(other) { Lt      => true, _ => false} }
    pure fn le(&&other: Sign) -> bool { match self.cmp(other) { Lt | Eq => true, _ => false} }
    pure fn ge(&&other: Sign) -> bool { match self.cmp(other) { Eq | Gt => true, _ => false} }
    pure fn gt(&&other: Sign) -> bool { match self.cmp(other) { Gt      => true, _ => false} }
}


trait ExtNum {
    pure fn abs() -> self;
    pure fn divmod(&&other: self) -> (self, self);

    pure fn is_zero() -> bool;
    pure fn is_not_zero() -> bool;
    pure fn is_positive() -> bool;
    pure fn is_negative() -> bool;
    pure fn is_nonpositive() -> bool;
    pure fn is_nonnegative() -> bool;

    pure fn to_uint() -> uint;
    pure fn to_str_radix(radix: uint) -> ~str;

    static pure fn zero() -> self;
    static pure fn one() -> self;
    static pure fn from_uint(n: uint) -> self;
    static fn parse_bytes(buf: &[u8], radix: uint) -> Option<self>;
    static fn from_str_radix(s: &str, radix: uint) -> Option<self>;
}
