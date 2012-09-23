use cmp::{ Ord, Eq };
use num::{ Num, from_int };

use extcmp::{ Cmp, Eq, Lt, Gt, ExtOrd };
use extnum::{ Sign, Minus, Zero, Plus, ExtNum, zero, one, from_uint, from_str_radix, parse_bytes };
use biguint::{ BigDigit, BigUint };

pub struct BigInt {
    priv sign: Sign,
    priv data: BigUint
}

pub pure fn from_biguint(sign: Sign, data: BigUint) -> BigInt {
    if sign == Zero || data.is_zero() {
        BigInt { sign: Zero, data: data }
    } else {
        BigInt { sign: sign, data: data }
    }
}
pub pure fn from_slice(sign: Sign, slice: &[BigDigit]) -> BigInt { from_biguint(sign, biguint::from_slice(slice)) }
pub pure fn from_at_vec(sign: Sign, at_vec: @[BigDigit]) -> BigInt { from_biguint(sign, biguint::from_at_vec(at_vec)) }

impl BigInt : ExtOrd {
    pure fn cmp(&&other: BigInt) -> Cmp {
        let ss = self.sign, os = other.sign;
        if ss < os { return Lt; }
        if ss > os { return Gt; }

        assert ss == os;
        match ss {
            Zero  => Eq,
            Plus  => self.data.cmp(other.data),
            Minus => self.data.cmp(other.data).neg(),
        }
    }
}

impl BigInt : Eq {
    #[inline(always)]
    pure fn eq(other: &BigInt) -> bool { match self.cmp(*other) { Eq => true, _ => false } }
    #[inline(always)]
    pure fn ne(other: &BigInt) -> bool { !self.eq(other) }
}

impl BigInt : Ord {
    #[inline(always)]
    pure fn lt(other: &BigInt) -> bool { match self.cmp(*other) { Lt      => true, _ => false} }
    #[inline(always)]
    pure fn le(other: &BigInt) -> bool { match self.cmp(*other) { Lt | Eq => true, _ => false} }
    #[inline(always)]
    pure fn ge(other: &BigInt) -> bool { match self.cmp(*other) { Eq | Gt => true, _ => false} }
    #[inline(always)]
    pure fn gt(other: &BigInt) -> bool { match self.cmp(*other) { Gt      => true, _ => false} }
}

impl BigInt : ToStr {
    fn to_str() -> ~str { self.to_str_radix(10) }
}

impl BigInt : Shl<uint, BigInt> {
    pure fn shl(rhs: &uint) -> BigInt { from_biguint(self.sign, self.data << *rhs) }
}

impl BigInt : Shr<uint, BigInt> {
    pure fn shr(rhs: &uint) -> BigInt { from_biguint(self.sign, self.data >> *rhs) }
}

impl BigInt : Num {
    pure fn add(&&other: BigInt) -> BigInt {
        match (self.sign, other.sign) {
            (Zero, _)      => other,
            (_,    Zero)   => self,
            (Plus, Plus)   => from_biguint(Plus, self.data.add(other.data)),
            (Plus, Minus)  => self.sub(-other),
            (Minus, Plus)  => other.sub(-self),
            (Minus, Minus) => -((-self).add(-other))
        }
    }
    pure fn sub(&&other: BigInt) -> BigInt {
        match (self.sign, other.sign) {
            (Zero, _)    => -other,
            (_,    Zero) => self,
            (Plus, Plus) => match self.data.cmp(other.data) {
                Lt => from_biguint(Minus, other.data.sub(self.data)),
                Eq => zero(),
                Gt => from_biguint(Plus, self.data.sub(other.data))
            },
            (Plus, Minus) => self.add(-other),
            (Minus, Plus) => -((-self).add(other)),
            (Minus, Minus) => (-other).sub(-self)
        }
    }
    pure fn mul(&&other: BigInt) -> BigInt {
        match (self.sign, other.sign) {
            (Zero, _)     | (_,     Zero)  => zero(),
            (Plus, Plus)  | (Minus, Minus) => from_biguint(Plus, self.data.mul(other.data)),
            (Plus, Minus) | (Minus, Plus)  => from_biguint(Minus, self.data.mul(other.data))
        }
    }
    pure fn div(&&other: BigInt) -> BigInt { self.divmod(other).first() }
    pure fn modulo(&&other: BigInt) -> BigInt { self.divmod(other).second() }
    pure fn neg() -> BigInt { from_biguint(self.sign.neg(), self.data) }

    pure fn to_int() -> int {
        match self.sign {
            Plus  => uint::min(self.to_uint(), int::max_value as uint) as int,
            Zero  => 0,
            Minus => uint::min((-self).to_uint(), (int::max_value as uint) + 1) as int
        }
    }
    static pure fn from_int(n: int) -> BigInt {
        if n > 0 { return from_biguint(Plus,  from_uint(n as uint)); }
        if n < 0 { return from_biguint(Minus, from_uint(uint::max_value - (n as uint) + 1)); }
        return zero();
    }
}

impl BigInt : ExtNum {
    pure fn abs() -> BigInt { from_biguint(Plus, self.data) }

    pure fn quot(&&other: BigInt) -> BigInt { self.quotrem(other).first() }
    pure fn rem(&&other: BigInt) -> BigInt { self.quotrem(other).second() }

    pure fn divmod(&&other: BigInt) -> (BigInt, BigInt) {
        // m.sign == other.sign
        let (d_ui, m_ui) = self.data.divmod(other.data);
        let d = from_biguint(Plus, d_ui), m = from_biguint(Plus, m_ui);
        match (self.sign, other.sign) {
            (_,    Zero)   => fail,
            (Plus, Plus)  | (Zero, Plus)  => (d, m),
            (Plus, Minus) | (Zero, Minus) => if m.is_zero() {
                (-d, zero())
            } else {
                // abs(s) = abs(o) d + m
                // s = abs(s)
                //   = abs(o) d + m
                //   = o(-d) + m
                //   = o(-d - 1) + (m + o)
                ((-d).sub(one()), m.add(other))
            },
            (Minus, Plus) => if m.is_zero() {
                (-d, zero())
            } else {
                // abs(s) = abs(o) d + m
                // s = -abs(s)
                //   = -abs(o) d - m
                //   = o (-d) - m
                //   = o (-d - 1) + (-m + o)
                ((-d).sub(one()), other.sub(m))
            },
            (Minus, Minus) => if m.is_zero() {
                (d, zero())
            } else {
                // abs(s) = abs(o) d + m
                // s = -abs(s)
                //   = -abs(o) d - m
                //   = o d - m
                (d, -m)
            }
        }
    }

    pure fn quotrem(&&other: BigInt) -> (BigInt, BigInt) {
        // m.sign == self.sign
        let (q_ui, r_ui) = self.data.quotrem(other.data);
        let q = from_biguint(Plus, q_ui), r = from_biguint(Plus, r_ui);
        match (self.sign, other.sign) {
            (_,    Zero)   => fail,
            (Plus, Plus)  | (Zero, Plus)  => (q, r),
            (Plus, Minus) | (Zero, Minus) => if r.is_zero() {
                (-q, zero())
            } else {
                // abs(s) = abs(o) q + r
                // s = abs(s)
                //   = abs(o) q + r
                //   = o(-q) + r
                (-q, r)
            },
            (Minus, Plus) => if r.is_zero() {
                (-q, zero())
            } else {
                // abs(s) = abs(o) q + r
                // s = -abs(s)
                //   = -abs(o) q - r
                //   = o (-q) - r
                (-q, -r)
            },
            (Minus, Minus) => if r.is_zero() {
                (q, zero())
            } else {
                // abs(s) = abs(o) q + r
                // s = -abs(s)
                //   = -abs(o) q - r
                //   = o q - r
                (q, -r)
            }
        }
    }

    #[inline(always)]
    pure fn is_zero() -> bool { self.sign == Zero }
    #[inline(always)]
    pure fn is_not_zero() -> bool { self.sign != Zero }
    #[inline(always)]
    pure fn is_positive() -> bool { self.sign == Plus }
    #[inline(always)]
    pure fn is_negative() -> bool { self.sign == Minus }
    #[inline(always)]
    pure fn is_nonpositive() -> bool { self.sign != Plus }
    #[inline(always)]
    pure fn is_nonnegative() -> bool { self.sign != Minus }

    pure fn to_uint() -> uint {
        match self.sign {
            Plus  => self.data.to_uint(),
            Zero  => 0,
            Minus => 0
        }
    }

    pure fn to_str_radix(radix: uint) -> ~str {
        match self.sign {
            Plus  => self.data.to_str_radix(radix),
            Zero  => ~"0",
            Minus => ~"-" + self.data.to_str_radix(radix)
        }
    }

    #[inline(always)]
    static pure fn zero() -> BigInt { from_biguint(Zero, zero()) }
    #[inline(always)]
    static pure fn one()  -> BigInt { from_biguint(Plus, one()) }

    static pure fn from_uint(n: uint) -> BigInt {
        if n == 0u { zero() }
        else       { from_biguint(Plus, from_uint(n)) }
    }

    static fn parse_bytes(buf: &[u8], radix: uint) -> Option<BigInt> {
        if buf.is_empty() { return None; }
        let mut sign  = Plus;
        let mut start = 0;
        if buf[0] == ('-' as u8) {
            sign  = Minus;
            start = 1;
        }
        return parse_bytes::<BigUint>(vec::view(buf, start, buf.len()), radix).map(|bu| from_biguint(sign, bu));
    }

    static fn from_str_radix(s: &str, radix: uint) -> Option<BigInt> { extnum::parse_bytes(str::to_bytes(s), radix) }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_from_biguint() {
        assert from_biguint(Plus, from_uint(1)) == BigInt { sign: Plus, data: from_uint(1) };
        assert from_biguint(Plus, zero()) == BigInt { sign: Zero, data: zero() };
        assert from_biguint(Minus, from_uint(1)) == BigInt { sign: Minus, data: from_uint(1) };
        assert from_biguint(Zero, from_uint(1)) == BigInt { sign: Zero, data: zero() };
    }

    #[test]
    fn test_cmp() {
        let uints = [ &[2], &[1, 1], &[2, 1], &[1, 1, 1] ].map(|data| biguint::from_slice(*data));
        let nums: ~[BigInt]
            = vec::reversed(uints).map(|bu| from_biguint(Minus, *bu))
            + [ zero() ]
            + uints.map(|bu| from_biguint(Plus, *bu));

        for uint::range(0, nums.len()) |i| {
            for uint::range(i, nums.len()) |j| {
                if i == j {
                    assert nums[i].cmp(nums[j]) == Eq;
                } else {
                    assert nums[i].cmp(nums[j]) == Lt;
                    assert nums[j].cmp(nums[i]) == Gt;
                }
            }
        }
    }

    #[test]
    fn test_convert_int() {
        fn check_conv(b: BigInt, i: int) {
            assert b == from_int(i);
            assert b.to_int() == i;
        }

        check_conv(zero(), 0);
        check_conv(one(), 1);

        check_conv(from_biguint(Plus, from_uint(int::max_value as uint)), int::max_value);
        assert from_biguint(Plus, from_uint(int::max_value as uint + 1)).to_int() == int::max_value;
        assert from_biguint(Plus, biguint::from_at_vec(@[1, 2, 3])).to_int() == int::max_value;

        check_conv(from_biguint(Minus, from_uint(-int::min_value as uint)), int::min_value);
        assert from_biguint(Minus, from_uint(-int::min_value as uint + 1)).to_int() == int::min_value;
        assert from_biguint(Minus, biguint::from_at_vec(@[1, 2, 3])).to_int() == int::min_value;
    }

    #[test]
    fn test_convert_uint() {
        fn check_conv(b: BigInt, u: uint) {
            assert b == from_uint(u);
            assert b.to_uint() == u;
        }

        check_conv(zero(), 0);
        check_conv(one(), 1);

        check_conv(from_biguint(Plus, from_uint(uint::max_value)), uint::max_value);
        assert from_biguint(Plus, biguint::from_at_vec(@[1, 2, 3])).to_uint() == uint::max_value;

        assert from_biguint(Minus, from_uint(uint::max_value)).to_uint() == 0;
        assert from_biguint(Minus, biguint::from_at_vec(@[1, 2, 3])).to_uint() == 0;
    }

    const sum_triples: &[(&[BigDigit], &[BigDigit], &[BigDigit])] = &[
        (&[],          &[],       &[]),
        (&[],          &[ 1],     &[ 1]),
        (&[ 1],        &[ 1],     &[ 2]),
        (&[ 1],        &[ 1,  1], &[ 2,  1]),
        (&[ 1],        &[-1],     &[ 0,  1]),
        (&[ 1],        &[-1, -1], &[ 0,  0, 1]),
        (&[-1, -1],    &[-1, -1], &[-2, -1, 1]),
        (&[ 1,  1, 1], &[-1, -1], &[ 0,  1, 2]),
        (&[ 2,  2, 1], &[-1, -2], &[ 1,  1, 2])
    ];

    #[test]
    fn test_add() {
        for sum_triples.each |elm| {
            let (aVec, bVec, cVec) = *elm;
            let a = from_slice(Plus, aVec);
            let b = from_slice(Plus, bVec);
            let c = from_slice(Plus, cVec);

            assert a.add(b) == c;
            assert b.add(a) == c;
            assert c.add(-a) == b;
            assert c.add(-b) == a;
            assert a.add(-c) == (-b);
            assert b.add(-c) == (-a);
            assert (-a).add(-b) == (-c);
            assert a.add(-a) == zero();
        }
    }

    #[test]
    fn test_sub() {
        for sum_triples.each |elm| {
            let (aVec, bVec, cVec) = *elm;
            let a = from_slice(Plus, aVec);
            let b = from_slice(Plus, bVec);
            let c = from_slice(Plus, cVec);

            assert c.sub(a) == b;
            assert c.sub(b) == a;
            assert (-b).sub(a) == (-c);
            assert (-a).sub(b) == (-c);
            assert b.sub(-a) == c;
            assert a.sub(-b) == c;
            assert (-c).sub(-a) == (-b);
            assert a.sub(a) == zero();
        }
    }

    const mul_triples: &[(&[BigDigit], &[BigDigit], &[BigDigit])] = &[
        (&[],               &[],               &[]),
        (&[],               &[ 1],             &[]),
        (&[ 2],             &[],               &[]),
        (&[ 1],             &[ 1],             &[1]),
        (&[ 2],             &[ 3],             &[ 6]),
        (&[ 1],             &[ 1,  1,  1],     &[1, 1,  1]),
        (&[ 1,  2,  3],     &[ 3],             &[ 3,  6,  9]),
        (&[ 1,  1,  1],     &[-1],             &[-1, -1, -1]),
        (&[ 1,  2,  3],     &[-1],             &[-1, -2, -2, 2]),
        (&[ 1,  2,  3,  4], &[-1],             &[-1, -2, -2, -2, 3]),
        (&[-1],             &[-1],             &[ 1, -2]),
        (&[-1, -1],         &[-1],             &[ 1, -1, -2]),
        (&[-1, -1, -1],     &[-1],             &[ 1, -1, -1, -2]),
        (&[-1, -1, -1, -1], &[-1],             &[ 1, -1, -1, -1, -2]),
        (&[-1/2 + 1],       &[ 2],             &[ 0,  1]),
        (&[0, -1/2 + 1],    &[ 2],             &[ 0,  0,  1]),
        (&[ 1,  2],         &[ 1,  2,  3],     &[1, 4,  7,  6]),
        (&[-1, -1],         &[-1, -1, -1],     &[1, 0, -1, -2, -1]),
        (&[-1, -1, -1],     &[-1, -1, -1, -1], &[1, 0,  0, -1, -2, -1, -1]),
        (&[ 0,  0,  1],     &[ 1,  2,  3],     &[0, 0,  1,  2,  3]),
        (&[ 0,  0,  1],     &[ 0,  0,  0,  1], &[0, 0,  0,  0,  0,  1])
    ];

    const divmod_quadruples: &[(&[BigDigit], &[BigDigit], &[BigDigit], &[BigDigit])] = &[
        (&[ 1],        &[ 2], &[],               &[1]),
        (&[ 1,  1],    &[ 2], &[-1/2+1],         &[1]),
        (&[ 1,  1, 1], &[ 2], &[-1/2+1, -1/2+1], &[1]),
        (&[ 0,  1],    &[-1], &[1],              &[1]),
        (&[-1, -1],    &[-2], &[2, 1],           &[3])
    ];

    #[test]
    fn test_mul() {
        for mul_triples.each |elm| {
            let (aVec, bVec, cVec) = *elm;
            let a = from_slice(Plus, aVec);
            let b = from_slice(Plus, bVec);
            let c = from_slice(Plus, cVec);

            assert a.mul(b) == c;
            assert b.mul(a) == c;

            assert (-a).mul(b) == -c;
            assert (-b).mul(a) == -c;
        }

        for divmod_quadruples.each |elm| {
            let (aVec, bVec, cVec, dVec) = *elm;
            let a = from_slice(Plus, aVec);
            let b = from_slice(Plus, bVec);
            let c = from_slice(Plus, cVec);
            let d = from_slice(Plus, dVec);

            assert a == b.mul(c).add(d);
            assert a == c.mul(b).add(d);
        }
    }
    
    #[test]
    fn test_divmod() {
        fn check_divmod_sub(a: BigInt, b: BigInt) {
            let (d, m) = a.divmod(b);
            if m.is_not_zero() {
                assert m.sign == b.sign;
            }
            assert m.abs() <= b.abs();
            assert a == b.mul(d).add(m);
        }
        fn check_divmod(a: BigInt, b: BigInt, c: BigInt, d: BigInt) {
            check_divmod_sub(a, b);
            check_divmod_sub(a, -b);
            check_divmod_sub(-a, b);
            check_divmod_sub(-a, -b);

            if d.is_zero() {
                assert a.divmod(b)     == (c, zero());
                assert (-a).divmod(b)  == (-c, zero());
                assert (a).divmod(-b)  == (-c, zero());
                assert (-a).divmod(-b) == (c, zero());
            } else {
                // a == bc + d
                assert a.divmod(b) == (c, d);
                // a == (-b)(-c - 1) + (d - b)
                assert a.divmod(-b) == ((-c).sub(one()), d.sub(b));
                // (-a) == b (-c - 1) + (b - d)
                assert (-a).divmod(b) == ((-c).sub(one()), b.sub(d));
                // (-a) == (-b)(c) - d
                assert (-a).divmod(-b) == (c, -d);
            }
        }
        for mul_triples.each |elm| {
            let (aVec, bVec, cVec) = *elm;
            let a = from_slice(Plus, aVec);
            let b = from_slice(Plus, bVec);
            let c = from_slice(Plus, cVec);

            if a != zero() { check_divmod(c, a, b, zero()); }
            if b != zero() { check_divmod(c, b, a, zero()); }
        }

        for divmod_quadruples.each |elm| {
            let (aVec, bVec, cVec, dVec) = *elm;
            let a = from_slice(Plus, aVec);
            let b = from_slice(Plus, bVec);
            let c = from_slice(Plus, cVec);
            let d = from_slice(Plus, dVec);

            if b != zero() {
                check_divmod(a, b, c, d);
            }
        }
    }


    #[test]
    fn test_quotrem() {
        fn check_quotrem_sub(a: BigInt, b: BigInt) {
            let (q, r) = a.quotrem(b);
            if r.is_not_zero() {
                assert r.sign == a.sign;
            }
            assert r.abs() <= b.abs();
            assert a == b.mul(q).add(r);
        }
        fn check_quotrem(a: BigInt, b: BigInt, c: BigInt, d: BigInt) {
            check_quotrem_sub(a, b);
            check_quotrem_sub(a, -b);
            check_quotrem_sub(-a, b);
            check_quotrem_sub(-a, -b);

            if d.is_zero() {
                assert a.quotrem(b)     == (c, zero());
                assert (-a).quotrem(b)  == (-c, zero());
                assert (a).quotrem(-b)  == (-c, zero());
                assert (-a).quotrem(-b) == (c, zero());
            } else {
                // a == bc + d
                assert a.quotrem(b) == (c, d);
                // a == (-b)(-c) + d
                assert a.quotrem(-b) == (-c, d);
                // (-a) == b (-c) + (-d)
                assert (-a).quotrem(b) == (-c, -d);
                // (-a) == (-b)(c) - d
                assert (-a).quotrem(-b) == (c, -d);
            }
        }
        for mul_triples.each |elm| {
            let (aVec, bVec, cVec) = *elm;
            let a = from_slice(Plus, aVec);
            let b = from_slice(Plus, bVec);
            let c = from_slice(Plus, cVec);

            if a != zero() { check_quotrem(c, a, b, zero()); }
            if b != zero() { check_quotrem(c, b, a, zero()); }
        }

        for divmod_quadruples.each |elm| {
            let (aVec, bVec, cVec, dVec) = *elm;
            let a = from_slice(Plus, aVec);
            let b = from_slice(Plus, bVec);
            let c = from_slice(Plus, cVec);
            let d = from_slice(Plus, dVec);

            if b != zero() {
                check_quotrem(a, b, c, d);
            }
        }
    }

    #[test]
    fn test_to_str_radix() {
        assert from_biguint(Plus, from_uint(10)).to_str_radix(10) == ~"10";
        assert one::<BigInt>().to_str_radix(10) == ~"1";
        assert zero::<BigInt>().to_str_radix(10) == ~"0";
        assert (-one::<BigInt>()).to_str_radix(10) == ~"-1";
        assert from_biguint(Minus, from_uint(10)).to_str_radix(10) == ~"-10";
    }


    #[test]
    fn test_from_str_radix() {
        assert from_biguint(Plus, from_uint(10)) == from_str_radix(~"10", 10).get();
        assert one::<BigInt>()== from_str_radix(~"1", 10).get();
        assert zero::<BigInt>() == from_str_radix(~"0", 10).get();
        assert (-one::<BigInt>()) == from_str_radix(~"-1", 10).get();
        assert from_biguint(Minus, from_uint(10)) == from_str_radix(~"-10", 10).get();

        assert from_str_radix::<BigInt>(~"Z", 10) == None;
        assert from_str_radix::<BigInt>(~"_", 2) == None;
        assert from_str_radix::<BigInt>(~"-1", 10) == Some(from_biguint(Minus, one()));
    }

    #[test]
    fn test_neg() {
        assert -from_at_vec(Plus,  @[1, 1, 1]) == from_at_vec(Minus, @[1, 1, 1]);
        assert -from_at_vec(Minus, @[1, 1, 1]) == from_at_vec(Plus,  @[1, 1, 1]);
        assert -zero::<BigInt>() == zero();
    }
}
