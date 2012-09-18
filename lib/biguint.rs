export BigUint, BigDigit, from_slice, from_at_vec;

use cmp::{ Ord, Eq };
use num::{ Num, from_int };

use extcmp::{ Cmp, Eq, Lt, Gt, ExtOrd };
use extnum::{ Sign, Minus, Zero, Plus, ExtNum, zero, one, from_uint, parse_bytes, from_str_radix };

#[cfg(target_arch = "x86")]
#[cfg(target_arch = "arm")]
type BigDigit = u16;

#[cfg(target_arch = "x86_64")]
type BigDigit = u32;

mod BigDigit {
    #[cfg(target_arch = "x86")]
    #[cfg(target_arch = "arm")]
    const bits: uint = 16;

    #[cfg(target_arch = "x86_64")]
    const bits: uint = 32;

    const base: uint = 1 << bits;

    priv const hi_mask: uint = (-1 as uint) << bits;
    priv const lo_mask: uint = (-1 as uint) >> bits;

    #[inline(always)]
    priv pure fn get_hi(n: uint) -> BigDigit { (n >> bits) as BigDigit }
    #[inline(always)]
    priv pure fn get_lo(n: uint) -> BigDigit { (n & lo_mask) as BigDigit }

    #[inline(always)]
    pure fn from_uint(n: uint) -> (BigDigit, BigDigit) { (get_hi(n), get_lo(n)) }

    #[inline(always)]
    pure fn join(hi: BigDigit, lo: BigDigit) -> uint { (lo as uint) | ((hi as uint) << bits) }
}

struct BigUint {
    priv data: @[BigDigit]
}

impl BigUint {
    pure fn ref_data<T>(f: fn(data: &[BigDigit]) -> T) -> T { f(self.data) }
}

pure fn from_slice(slice: &[BigDigit]) -> BigUint {
    let end = slice.rposition(|n| n != 0).map_default(0, |p| p + 1);
    return BigUint { data: @[] + vec::view(slice, 0, end) };
}

pure fn from_at_vec(at_vec: @[BigDigit]) -> BigUint {
    let end = at_vec.rposition(|n| n != 0).map_default(0, |p| p + 1) ;
    return BigUint {
        data: if end == at_vec.len() {
            at_vec
        } else {
            @[] + vec::view(at_vec, 0, end)
        }
    };
}

priv pure fn get_radix_base(radix: uint) -> (uint, uint) {
    assert 1 < radix && radix <= 16;
    match radix {
        2  => (4294967296, 32),
        3  => (3486784401, 20),
        4  => (4294967296, 16),
        5  => (1220703125, 13),
        6  => (2176782336, 12),
        7  => (1977326743, 11),
        8  => (1073741824, 10),
        9  => (3486784401, 10),
        10 => (1000000000, 9),
        11 => (2357947691, 9),
        12 => (429981696,  8),
        13 => (815730721,  8),
        14 => (1475789056, 8),
        15 => (2562890625, 8),
        16 => (4294967296, 8),
        _  => fail
    }
}

impl BigUint : ExtOrd {
    pure fn cmp(&&other: BigUint) -> Cmp {
        let sLen = self.data.len(), oLen = other.data.len();
        if sLen < oLen { return Lt; }
        if sLen > oLen { return Gt; }

        match vec::zip_slice(self.data, other.data).rfind(|z| z.first() != z.second()) {
            Some((e1, e2)) => if e1 < e2 { Lt } else { Gt },
            None           => Eq
        }
    }
}

impl BigUint : Eq {
    #[inline(always)]
    pure fn eq(&&other: BigUint) -> bool { match self.cmp(other) { Eq => true, _ => false } }
    #[inline(always)]
    pure fn ne(&&other: BigUint) -> bool { !self.eq(other) }
}

impl BigUint : Ord {
    #[inline(always)]
    pure fn lt(&&other: BigUint) -> bool { match self.cmp(other) { Lt      => true, _ => false} }
    #[inline(always)]
    pure fn le(&&other: BigUint) -> bool { match self.cmp(other) { Lt | Eq => true, _ => false} }
    #[inline(always)]
    pure fn ge(&&other: BigUint) -> bool { match self.cmp(other) { Eq | Gt => true, _ => false} }
    #[inline(always)]
    pure fn gt(&&other: BigUint) -> bool { match self.cmp(other) { Gt      => true, _ => false} }
}

impl BigUint: Shl<uint, BigUint> {
    pure fn shl(&&rhs: uint) -> BigUint {
        let n_unit = rhs / BigDigit::bits;
        let n_bits = rhs % BigDigit::bits;

        let data = if n_bits == 0 {
            self.data
        } else {
            let mut carry = 0;
            let result = do at_vec::map(self.data) |elt| {
                let (hi, lo) = BigDigit::from_uint((elt as uint) << n_bits | (carry as uint));
                carry = hi;
                lo
            };
            if carry != 0 { result + [carry] } else { result }
        };
        return from_at_vec(if data.is_empty() { data } else { at_vec::from_elem(n_unit, 0) + data });
    }
}

impl BigUint: Shr<uint, BigUint> {
    pure fn shr(&&rhs: uint) -> BigUint {
        let n_unit = rhs / BigDigit::bits;
        let n_bits = rhs % BigDigit::bits;

        let data = if n_bits == 0 {
            self.data
        } else {
            let mut borrow = 0;
            let mut result = @[];
            for vec::reach(self.data) |elt| {
                result = @[ (elt >> n_bits) | borrow ] + result;
                borrow = elt << (uint::bits - n_bits);
            }
            result
        };
        return from_at_vec(if data.len() < n_unit { @[] } else { @[] + vec::view(data, n_unit, data.len()) });
    }
}

impl BigUint : Num {
    pure fn add(&&other: BigUint) -> BigUint {
        let mut carry = 0;
        let sum = do at_vec::map(util::zip_default(self.data, other.data, (0, 0))) |elm| {
            let (ai, bi) = elm;
            let (hi, lo) = BigDigit::from_uint((ai as uint) + (bi as uint) + (carry as uint));
            carry = hi;
            lo
        };
        return from_at_vec(if carry == 0 { sum } else { sum + [carry]});
    }

    pure fn sub(&&other: BigUint) -> BigUint {
        let mut borrow = 0;
        let diff = do at_vec::map(util::zip_default(self.data, other.data, (0, 0))) |elm| {
            let (ai, bi) = elm;
            let (hi, lo) = BigDigit::from_uint((BigDigit::base) + (ai as uint) - (bi as uint) - (borrow as uint));
            borrow = if hi == 0 { 1 }  else { 0 };
            lo
        };
        assert borrow == 0;
        return from_at_vec(diff);
    }

    pure fn mul(&&other: BigUint) -> BigUint {
        pure fn mul_uint(a: BigUint, n: BigDigit) -> BigUint {
            if n == 0 { return zero(); }
            if n == 1 { return a; }

            let mut carry = 0;
            let prod = do at_vec::map(a.data) |ai| {
                let (hi, lo) = BigDigit::from_uint((ai as uint) * (n as uint) + (carry as uint));
                carry = hi;
                lo
            };
            return from_at_vec(if carry == 0 { prod } else { prod + [carry]});
        }

        pure fn cut_at(a: BigUint, n: uint) -> (BigUint, BigUint) {
            let mid = uint::min(a.data.len(), n);
            return (from_slice(vec::view(a.data, mid, a.data.len())),
                    from_slice(vec::view(a.data, 0, mid)));
        }

        pure fn sub_sign(a: BigUint, b: BigUint) -> (int, BigUint) {
            match a.cmp(b) {
                Eq => ( 0, zero()),
                Lt => (-1, b - a),
                Gt => ( 1, a - b)
            }
        }

        pure fn prod_sign(a: (int, BigUint), b: (int, BigUint)) -> (int, BigUint) {
            (a.first() * b.first(), a.second() * b.second())
        }

        let sLen = self.data.len(), oLen = other.data.len();
        if sLen == 0 || oLen == 0 { return zero(); }
        if sLen == 1 { return mul_uint(other, self.data[0]); }
        if oLen == 1 { return mul_uint(self, other.data[0]); }

        let spLen = uint::max(sLen, oLen) / 2;
        let (sHi, sLo) = cut_at(self, spLen);
        let (oHi, oLo) = cut_at(other, spLen);
        let ll = sLo * oLo;
        let hh = sHi * oHi;
        let mm = match prod_sign(sub_sign(sHi, sLo), sub_sign(oHi, oLo)) {
            (-1, n) => hh + ll + n,
            ( 1, n) => hh + ll - n,
            ( 0, _) => hh + ll,
            _ => fail
        };

        return ll + (mm << spLen * BigDigit::bits) + (hh << spLen * BigDigit::bits * 2);
    }

    pure fn div(&&other: BigUint) -> BigUint    { self.divmod(other).first()  }
    pure fn modulo(&&other: BigUint) -> BigUint { self.divmod(other).second() }

    pure fn neg() -> BigUint { fail }

    pure fn to_int() -> int {
        uint::min(self.to_uint(), int::max_value as uint) as int
    }

    static pure fn from_int(n: int) -> BigUint {
        return if (n < 0) { zero() } else { from_uint(n as uint) };
    }
}

impl BigUint : ExtNum {
    pure fn abs() -> BigUint { self }

    pure fn divmod(&&other: BigUint) -> (BigUint, BigUint) {
        pure fn div_estimate(a: BigUint, b: BigUint, n: uint) -> (BigUint, BigUint, BigUint) {
            if a.data.len() < n { return (zero(), zero(), a); }

            let an = vec::view(a.data, a.data.len() - n, a.data.len());
            let bn = b.data.last();
            let mut d = ~[];
            let mut carry = 0;
            for vec::reach(an) |elt| {
                let ai = BigDigit::join(carry, elt);
                let di = ai / (bn as uint);
                assert di < BigDigit::base;
                carry = (ai % (bn as uint)) as BigDigit;
                d = ~[di as BigDigit] + d;
            }

            let shift = ((a.data.len() - an.len()) - (b.data.len() - 1)) * BigDigit::bits;
            return (from_slice(d) << shift, one::<BigUint>() << shift, b << shift);
        }

        pure fn divmod_inner(a: BigUint, b: BigUint) -> (BigUint, BigUint) {
            let mut r = a;
            let mut d = zero::<BigUint>();
            let mut n = 1;
            while r >= b {
                let mut (d0, dUnit, bUnit) = div_estimate(r, b, n);
                let mut prod = b * d0;
                while prod > r {
                    d0   -= dUnit;
                    prod -= bUnit;
                }
                if d0 == zero() {
                    n = 2;
                    loop;
                }
                n = 1;
                d += d0;
                r -= prod;
            }
            return (d, r);
        }

        let sLen = self.data.len(), oLen = other.data.len();
        if oLen == 0 { fail }
        if sLen == 0 { return (zero(), zero()); }
        if other == from_uint(1) { return (self, zero()); }

        match self.cmp(other) {
            Lt => return (zero(), self),
            Eq => return (one(), zero()),
            Gt => {} // Do nothing
        }

        let mut shift = 0;
        let mut n = other.data.last();
        while n < (1 << BigDigit::bits - 2) {
            n <<= 1;
            shift += 1;
        }
        let (d, m) = divmod_inner(self << shift, other << shift);
        return (d, m >> shift);
    }

    pure fn to_uint() -> uint {
        match self.data.len() {
            0 => 0,
            1 => self.data[0] as uint,
            2 => BigDigit::join(self.data[1], self.data[0]),
            _ => uint::max_value
        }
    }

    pure fn to_str_radix(radix: uint) -> ~str {
        assert 1 < radix && radix <= 16;

        pure fn convert_base(n: BigUint, base: uint) -> @[BigDigit] {
            if base == BigDigit::base { return n.data; }
            let divider    = from_uint(base);
            let mut result = @[];
            let mut r      = n;
            while r > divider {
                let (d, r0) = r.divmod(divider);
                result += @[ r0.to_uint() as BigDigit ];
                r = d;
            }
            if r != zero() {
                result += @[ r.to_uint() as BigDigit ];
            }
            return result;
        }

        pure fn fill_concat(v: &[BigDigit], radix: uint, l: uint) -> ~str {
            if v.is_empty() { return ~"0" }
            str::trim_left_chars(str::concat(vec::reversed(v).map(|n| {
                let s = uint::to_str(n as uint, radix);
                str::from_chars(vec::from_elem(l - s.len(), '0')) + s
            })), ['0'])
        }

        let (base, maxLen) = get_radix_base(radix);
        return fill_concat(convert_base(self, base), radix, maxLen);
    }

    static pure fn zero() -> BigUint { from_at_vec(@[]) }
    static pure fn one()  -> BigUint { from_at_vec(@[1]) }

    static pure fn from_uint(n: uint) -> BigUint {
        match BigDigit::from_uint(n) {
            (0,  0)  => zero(),
            (0,  n0) => from_at_vec(@[n0]),
            (n1, n0) => from_at_vec(@[n0, n1])
        }
    }

    static fn parse_bytes(buf: &[u8], radix: uint) -> Option<BigUint> {
        let (base, width) = get_radix_base(radix);
        let base_num: BigUint = from_uint(base);

        let mut end             = buf.len();
        let mut n: BigUint      = zero();
        let mut power: BigUint  = one();
        loop {
            let start = uint::max(end, width) - width;
            match uint::parse_bytes(vec::view(buf, start, end), radix) {
                Some(d) => n += from_uint::<BigUint>(d) * power,
                None    => return None
            }
            if end <= width {
                return Some(n);
            }
            end -= width;
            power *= base_num;
        }
    }

    static fn from_str_radix(s: &str, radix: uint) -> Option<BigUint> { parse_bytes(str::to_bytes(s), radix) }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_from_slice() {
        let pairs = [
            (&[1],                &[1]),
            (&[0, 0],             &[]),
            (&[1, 2, 0, 0],       &[1, 2]),
            (&[0, 0, 1, 2, 0, 0], &[0, 0, 1, 2]),
            (&[-1],               &[-1])
        ];
        for pairs.each |p| {
            from_slice(p.first()).ref_data(|data| assert data == p.second());
        }
    }

    #[test]
    fn test_cmp() {
        let data = [ &[], &[1], &[2], &[-1], &[0, 1], &[2, 1], &[1, 1, 1]  ].map(from_slice);
        for data.eachi |i, ni| {
            for vec::view(data, i, data.len()).eachi |j0, nj| {
                let j = j0 + i;
                if i == j {
                    assert ni.cmp(nj) == Eq;
                    assert nj.cmp(ni) == Eq;
                } else {
                    assert ni.cmp(nj) == Lt;
                    assert nj.cmp(ni) == Gt;
                }
            }
        }
    }

    #[test]
    fn test_shl() {
        assert from_at_vec(@[]) << 3 == from_at_vec(@[]);
        assert from_at_vec(@[1, 1, 1]) << 3 == from_at_vec(@[1 << 3, 1 << 3, 1 << 3]);

        assert from_at_vec(@[1 << (BigDigit::bits - 2)]) << 2 == from_at_vec(@[0, 1]);
        assert from_at_vec(@[1 << (BigDigit::bits - 2)]) << 3 == from_at_vec(@[0, 2]);
        assert from_at_vec(@[1 << (BigDigit::bits - 2)]) << (3 + BigDigit::bits) == from_at_vec(@[0, 0, 2]);

        assert from_at_vec(@[0x7654_3210, 0xfedc_ba98, 0x7654_3210, 0xfedc_ba98]) << 4 ==
            from_at_vec(@[0x6543_2100, 0xedcb_a987, 0x6543_210f, 0xedcb_a987, 0xf]);
        assert from_at_vec(@[0x2222_1111, 0x4444_3333, 0x6666_5555, 0x8888_7777]) << 16 ==
            from_at_vec(@[0x1111_0000, 0x3333_2222, 0x5555_4444, 0x7777_6666, 0x8888]);
    }

    #[test]
    fn test_shr() {
        assert from_at_vec(@[]) >> 3 == from_at_vec(@[]);
        assert from_at_vec(@[1, 1, 1]) >> 3 == from_at_vec(@[1 << (BigDigit::bits - 3), 1 << (BigDigit::bits - 3)]);

        assert from_at_vec(@[1 << 2]) >> 2 == from_at_vec(@[1]);
        assert from_at_vec(@[1, 2]) >> 3 == from_at_vec(@[1 << (BigDigit::bits - 2)]);
        assert from_at_vec(@[1, 1, 2]) >> (3 + BigDigit::bits) == from_at_vec(@[1 << (BigDigit::bits - 2)]);

        assert from_at_vec(@[0x6543_2100, 0xedcb_a987, 0x6543_210f, 0xedcb_a987, 0xf]) >> 4 ==
            from_at_vec(@[0x7654_3210, 0xfedc_ba98, 0x7654_3210, 0xfedc_ba98]);

        assert from_at_vec(@[0x1111_0000, 0x3333_2222, 0x5555_4444, 0x7777_6666, 0x8888]) >> 16 ==
            from_at_vec(@[0x2222_1111, 0x4444_3333, 0x6666_5555, 0x8888_7777]);
    }

    #[test]
    fn test_convert_int() {
        fn check_conv(b: BigUint, i: int) {
            assert b == from_int(i);
            assert b.to_int() == i;
        }

        check_conv(zero(), 0);
        check_conv(from_at_vec(@[1]), 1);

        check_conv(from_at_vec(@[-1]),     (uint::max_value >> BigDigit::bits) as int);
        check_conv(from_at_vec(@[ 0,  1]), ((uint::max_value >> BigDigit::bits) + 1) as int);
        check_conv(from_at_vec(@[-1, -1 >> 1]), int::max_value);

        assert from_at_vec(@[0, -1]).to_int() == int::max_value;
        assert from_at_vec(@[0, 0, 1]).to_int() == int::max_value;
        assert from_at_vec(@[0, 0, -1]).to_int() == int::max_value;
    }

    #[test]
    fn test_convert_uint() {
        fn check_conv(b: BigUint, u: uint) {
            assert b == from_uint(u);
            assert b.to_uint() == u;
        }

        check_conv(zero(),                0);
        check_conv(from_at_vec(@[ 1]),     1);
        check_conv(from_at_vec(@[-1]),     uint::max_value >> BigDigit::bits);
        check_conv(from_at_vec(@[ 0,  1]), (uint::max_value >> BigDigit::bits) + 1);
        check_conv(from_at_vec(@[ 0, -1]), uint::max_value << BigDigit::bits);
        check_conv(from_at_vec(@[-1, -1]), uint::max_value);

        assert from_at_vec(@[0, 0, 1]).to_uint()  == uint::max_value;
        assert from_at_vec(@[0, 0, -1]).to_uint() == uint::max_value;
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
            let (aVec, bVec, cVec) = elm;
            let a = from_slice(aVec);
            let b = from_slice(bVec);
            let c = from_slice(cVec);

            assert a + b == c;
            assert b + a == c;
        }
    }

    #[test]
    fn test_sub() {
        for sum_triples.each |elm| {
            let (aVec, bVec, cVec) = elm;
            let a = from_slice(aVec);
            let b = from_slice(bVec);
            let c = from_slice(cVec);

            assert c - a == b;
            assert c - b == a;
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
            let (aVec, bVec, cVec) = elm;
            let a = from_slice(aVec);
            let b = from_slice(bVec);
            let c = from_slice(cVec);

            assert a * b == c;
            assert b * a == c;
        }

        for divmod_quadruples.each |elm| {
            let (aVec, bVec, cVec, dVec) = elm;
            let a = from_slice(aVec);
            let b = from_slice(bVec);
            let c = from_slice(cVec);
            let d = from_slice(dVec);

            assert a == b * c + d;
            assert a == c * b + d;
        }
    }

    #[test]
    fn test_divmod() {
        for mul_triples.each |elm| {
            let (aVec, bVec, cVec) = elm;
            let a = from_slice(aVec);
            let b = from_slice(bVec);
            let c = from_slice(cVec);

            if a != zero() { assert c / a == b; }
            if b != zero() { assert c / b == a; }
        }

        for divmod_quadruples.each |elm| {
            let (aVec, bVec, cVec, dVec) = elm;
            let a = from_slice(aVec);
            let b = from_slice(bVec);
            let c = from_slice(cVec);
            let d = from_slice(dVec);

            if b != zero() { assert a.divmod(b) == (c, d); }
        }
    }

    fn to_str_pairs() -> ~[ (BigUint, ~[(uint, ~str)]) ] {
        ~[( zero(), ~[
            (2, ~"0"), (3, ~"0")
        ]), ( from_slice([ 0xff ]), ~[
            (2,  ~"11111111"),
            (3,  ~"100110"),
            (4,  ~"3333"),
            (5,  ~"2010"),
            (6,  ~"1103"),
            (7,  ~"513"),
            (8,  ~"377"),
            (9,  ~"313"),
            (10, ~"255"),
            (11, ~"212"),
            (12, ~"193"),
            (13, ~"168"),
            (14, ~"143"),
            (15, ~"120"),
            (16, ~"ff")
        ]), ( from_slice([ 0xfff ]), ~[
            (2,  ~"111111111111"),
            (4,  ~"333333"),
            (16, ~"fff")
        ]), ( from_slice([ 1, 2 ]), ~[
            (2,  ~"10" + str::from_chars(vec::from_elem(31, '0')) + "1"),
            (4,  ~"2"  + str::from_chars(vec::from_elem(15, '0')) + "1"),
            (10, ~"8589934593"),
            (16, ~"2"  + str::from_chars(vec::from_elem(7, '0')) + "1")
        ]), (from_slice([ 1, 2, 3 ]), ~[
            (2,  ~"11" + str::from_chars(vec::from_elem(30, '0')) + "10" + str::from_chars(vec::from_elem(31, '0')) + "1"),
            (4,  ~"3"  + str::from_chars(vec::from_elem(15, '0')) + "2"  + str::from_chars(vec::from_elem(15, '0')) + "1"),
            (10, ~"55340232229718589441"),
            (16, ~"3"  + str::from_chars(vec::from_elem(7, '0')) + "2"  + str::from_chars(vec::from_elem(7, '0')) + "1")
        ])]
    }

    #[test]
    fn test_to_str_radix() {
        for to_str_pairs().each |num_pair| {
            let (n, rs) = num_pair;
            for rs.each |str_pair| {
                let (radix, str) = str_pair;
                assert n.to_str_radix(radix) == str;
            }
        }
    }

    #[test]
    fn test_from_str_radix() {
        for to_str_pairs().each |num_pair| {
            let (n, rs) = num_pair;
            for rs.each |str_pair| {
                let (radix, str) = str_pair;
                assert Some(n) == from_str_radix(str, radix);
            }
        }

        assert from_str_radix::<BigUint>(~"Z", 10) == None;
        assert from_str_radix::<BigUint>(~"_", 2) == None;
    }

    #[test]
    fn test_factor() {
        fn factor(n: uint) -> BigUint {
            let mut f: BigUint = one();
            for uint::range(2, n + 1) |i| {
                f *= from_uint(i);
            }
            return f;
        }

        assert factor(3) == from_str_radix(~"6", 10).get();
        assert factor(10) == from_str_radix(~"3628800", 10).get();
        assert factor(20) == from_str_radix(~"2432902008176640000", 10).get();
        assert factor(30) == from_str_radix(~"265252859812191058636308480000000", 10).get();
    }
}
