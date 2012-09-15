export bigint;

use extcmp::{ Cmp, Eq, Lt, Gt, ExtOrd };
use extnum::{ Sign, Minus, Zero, Plus, ExtNum, zero, from_uint, from_str_radix };
use cmp::{ Ord, Eq };
use to_str::{ ToStr };
use from_str::{ FromStr };

mod UintVec {
    priv pure fn reduce_zero_slice(v: &[uint]) -> &[uint] {
        match v.rposition(|n| n != 0) {
          Some(p) => vec::view(v, 0, p + 1),
          None    => vec::view(v, 0, 0)
        }
    }

    priv pure fn reduce_zero(v: &[uint]) -> @[uint] {
        vec_from_slice(reduce_zero_slice(v))
    }

    priv pure fn cut_at_slice(v: &[uint], n: uint) -> (&[uint], &[uint]) {
        let spAt = uint::min(v.len(), n);
        (reduce_zero_slice(vec::view(v, 0, spAt)), vec::view(v, spAt, v.len()))
    }

    priv pure fn shl_unit(a: &[uint], n: uint) -> @[uint] {
        if a.is_empty() { return @[] }
        return at_vec::from_elem(n, 0) + a;
    }

    pure fn shl(a: &[uint], n: uint) -> @[uint] {
        let (n_unit, n_bits) = arith::div_rem(n, uint::bits);
        let mut carry = 0;
        let mut result = @[];
        if n_bits == 0 {
            result += a;
        } else {
            for a.each |elt| {
                result += [ (elt << n_bits) | carry ];
                carry   = (elt >> (uint::bits - n_bits));
            }
            if carry != 0 {
                result += [ carry ];
            }
        }
        return shl_unit(result, n_unit);
    }

    priv pure fn shr_unit(a: &[uint], n: uint) -> @[uint] {
        if a.is_empty() || a.len() < n { return @[] }
        return @[] + vec::view(a, n, a.len())
    }

    pure fn shr(a: &[uint], n: uint) -> @[uint] {
        let (n_unit, n_bits) = arith::div_rem(n, uint::bits);
        let mut carry = 0;
        let mut result = @[];
        for vec::reach(a) |elt| {
            result = @[ (elt >> n_bits) | carry ] + result;
            carry = elt << (uint::bits - n_bits);
        }
        return shr_unit(reduce_zero_slice(result), n_unit);
    }

    pure fn cmp(a: &[uint], b: &[uint]) -> Cmp {
        let (aLen, bLen) = (a.len(), b.len());
        if aLen < bLen { return Lt; }
        if aLen > bLen { return Gt; }
        match vec::zip_slice(a, b).rfind(|z| z.first() != z.second()) {
          Some((e1, e2)) => if e1 < e2 { Lt } else { Gt },
          None           => Eq
        }
    }

    pure fn add(a: &[uint], b: &[uint]) -> @[uint] {
        let mut sumArr = @[];
        let mut carry  = 0;
        for util::zip_default(a, b, (0, 0)).each |elm| {
            let (sum, nextCarry) = arith::add_carry_multi(&[elm.first(), elm.second(), carry]);
            sumArr += [ sum ];
            carry   = nextCarry;
        }
        if carry == 0 {
            return sumArr;
        } else {
            return sumArr + [carry];
        }
    }

    pure fn sub(a: &[uint], b: &[uint]) -> @[uint] {
        let mut diffArr = ~[];
        let mut borrow  = 0;
        for util::zip_default(a, b, (0, 0)).each |elm| {
            let (diff, nextBorrow) = arith::sub_borrow_multi(elm.first(), &[elm.second(), borrow]);
            diffArr += [ diff ];
            borrow   = nextBorrow;
        }
        assert borrow == 0;
        return reduce_zero(diffArr);
    }

    priv pure fn mul_uint(v: &[uint], n: uint) -> @[uint] {
        match n {
          0 => vec_from_uint(0),
          1 => vec_from_slice(v),
          _ => {
            let mut prodArr = @[];
            let mut carry   = 0;
            for v.each |elm| {
                let (prod, nextCarry) = arith::add_carry2(arith::mul_carry(elm, n), carry);
                prodArr += [ prod ];
                carry    = nextCarry;
            }
            if carry == 0 {
                prodArr
            } else {
                prodArr + [ carry ]
            }
          }
        }
    }

    pure fn mul(a: &[uint], b: &[uint]) -> @[uint] {
        match (a.len(), b.len()) {
          (0, _) | (_, 0) => vec_from_uint(0),
          (1, _) => mul_uint(b, to_uint(a)),
          (_, 1) => mul_uint(a, to_uint(b)),
          (l1, l2) => {
            let spLen = uint::max(l1, l2) / 2;
            let (aLo, aHi) = cut_at_slice(a, spLen);
            let (bLo, bHi) = cut_at_slice(b, spLen);

            let llVec = mul(aLo, bLo);
            let hhVec = mul(aHi, bHi);
            let mmTmp = from_slice::<BigInt>(Plus, add(hhVec, llVec))
                - (from_slice::<BigInt>(Plus, aHi) - from_slice(Plus, aLo)) * (from_slice::<BigInt>(Plus, bHi) - from_slice(Plus, bLo));

            let ll = from_slice(Plus, llVec);
            let mm = BigInt { nums: shl_unit(mmTmp.nums, spLen), .. mmTmp };
            let hh = from_slice::<BigInt>(Plus, shl_unit(hhVec, spLen * 2));

            return (hh + mm + ll).nums;
          }
        }
    }

    priv pure fn div_estimate(a: &[uint], b: &[uint], n: uint) -> (@[uint], @[uint], @[uint]) {
        assert a.is_not_empty();
        assert b.is_not_empty();
        assert cmp(a, b) != Lt;
        if a.len() < n {
            return (vec_from_uint(0), vec_from_uint(0), vec_from_slice(a));
        }

        let an = vec::view(a, a.len() - n, a.len());
        let bn = b.last();
        let mut unit_zero_len = (a.len() - an.len()) - (b.len() - 1);
        let mut d = ~[];
        let mut carry = 0;
        for vec::reach(an) |ai| {
            let mut di = (uint::max_value / bn + (1 + uint::max_value % bn) / bn) * carry + ai / bn;
            let mut prod = mul_uint([di], bn);
            let mut diff = sub([ai, carry], prod);
            while cmp(diff, [ bn ]) == Gt {
                diff = sub(diff, [ bn ]);
                di += 1;
            }
            assert diff.len() <= 1;
            carry = to_uint(diff);
            d = ~[ di ] + d;
        }
        let zeros = at_vec::from_elem(unit_zero_len, 0);
        return (reduce_zero(zeros + d), zeros + [1], zeros + b);
    }

    priv pure fn divmod_inner(a: &[uint], b: &[uint]) -> (@[uint], @[uint]) {
        let mut r = vec_from_slice(a);
        let mut d = vec_from_uint(0);
        let mut n = 1;
        while cmp(r, b) != Lt {
            let mut (d0, dUnit, bUnit) = div_estimate(r, b, n);
            let mut b_d0 = mul(b, d0);
            while cmp(b_d0, r) == Gt {
                d0   = sub(d0, dUnit);
                b_d0 = sub(b_d0, bUnit);
            }
            if cmp(d0, vec_from_uint(0)) == Eq {
                assert n < 2;
                n = 2;
                loop;
            }
            n = 1;
            d = add(d, d0);
            r = sub(r, b_d0);
        }
        return (d, r);
    }
    
    pure fn divmod(a: &[uint], b: &[uint]) -> (@[uint], @[uint]) {
        match (a.len(), b.len()) {
          (_, 0) => fail,
          (0, _) => (vec_from_uint(0), vec_from_uint(0)),
          (_, _) if b == &[1] => (vec_from_slice(a), vec_from_uint(0)),
          (l1, l2) if l1 < l2 => (vec_from_uint(0), vec_from_slice(a)),
          (_, _) => {
            match cmp(a, b) {
              Lt => return (vec_from_uint(0), vec_from_slice(a)),
              Eq => return (vec_from_uint(1), vec_from_uint(0)),
              Gt => { } // Do nothing
            }
            let mut shift = 0;
            let mut bn = b.last();
            while bn < (1 << (uint::bits - 2)) {
                bn <<= 1;
                shift += 1;
            }
            let (d, m) = divmod_inner(shl(a, shift), shl(b, shift));
            return (d, shr(m, shift));
          }
        }
    }

    pure fn to_int(n: &[uint]) -> int { if n.is_empty() { 0 } else { n[0] as int } }
    pure fn to_uint(n: &[uint]) -> uint { if n.is_empty() { 0 } else { n[0] } }

    pure fn vec_from_slice(s: &[uint]) -> @[uint] { @[] + reduce_zero_slice(s) }
    pure fn vec_from_uint(n: uint) -> @[uint] { reduce_zero(~[n]) }

    pure fn vec_to_str_bin(v: &[uint]) -> ~str {
        str::trim_left_chars(str::connect(vec::reversed(v).map(|elt| #fmt("%064t", elt)), "_"), ~['0'])
    }
    pure fn vec_to_str_oct(v: &[uint]) -> ~str {
        str::trim_left_chars(str::connect(vec::reversed(v).map(|elt| #fmt("%064o", elt)), "_"), ~['0'])
    }
    pure fn vec_to_str_hex(v: &[uint]) -> ~str {
        str::trim_left_chars(str::connect(vec::reversed(v).map(|elt| #fmt("%064x", elt)), "_"), ~['0'])
    }
    pure fn vec_to_str_radix(v: &[uint], radix: uint) -> ~str {
        assert 1 < radix && radix <= 16;
        match radix {
            2 => return vec_to_str_bin(v),
            8 => return vec_to_str_oct(v),
            16 => return vec_to_str_hex(v),
            _ => fail
        }
    }
}


trait BigIntGen {
    static pure fn from_slice(s: Sign, n: &[uint]) -> self;
}

struct BigInt {
    sign: Sign,
    nums: @[uint]
}

impl BigInt: ExtOrd {
    pure fn cmp(&&other: BigInt) -> Cmp {
        let ss = self.sign, os = other.sign;
        if ss < os { return Lt; }
        if ss > os { return Gt; }

        assert ss == os;
        match ss {
          Zero  => Eq,
          Plus  => UintVec::cmp(self.nums, other.nums),
          Minus => UintVec::cmp(self.nums, other.nums).neg()
        }
    }
}

impl BigInt : Eq {
    pure fn eq(&&other: BigInt) -> bool { match self.cmp(other) { Eq => true, _ => false } }
    pure fn ne(&&other: BigInt) -> bool { !self.eq(other) }
}

impl BigInt : Ord {
    pure fn lt(&&other: BigInt) -> bool { match self.cmp(other) { Lt      => true, _ => false} }
    pure fn le(&&other: BigInt) -> bool { match self.cmp(other) { Lt | Eq => true, _ => false} }
    pure fn ge(&&other: BigInt) -> bool { match self.cmp(other) { Eq | Gt => true, _ => false} }
    pure fn gt(&&other: BigInt) -> bool { match self.cmp(other) { Gt      => true, _ => false} }
}

impl BigInt: Shl<uint, BigInt> {
    pure fn shl(&&rhs: uint) -> BigInt {
        BigInt { nums: UintVec::shl(self.nums, rhs), .. self }
    }
}

impl BigInt: Shr<uint, BigInt> {
    pure fn shr(&&rhs: uint) -> BigInt {
        BigInt { nums: UintVec::shr(self.nums, rhs), .. self }
    }
}

impl BigInt : Num {
    pure fn add(&&other: BigInt) -> BigInt {
        match (self.sign, other.sign) {
          (Zero,  _    ) => other,
          (_,     Zero ) => self,
          (Plus,  Plus ) => from_slice(Plus, UintVec::add(self.nums, other.nums)),
          (Plus,  Minus) =>   self    - (-other),
          (Minus, Plus ) =>   other   - (-self),
          (Minus, Minus) => -((-self) + (-other))
        }
    }

    pure fn sub(&&other: BigInt) -> BigInt {
        match (self.sign, other.sign) {
          (Zero, _   ) => -other,
          (_,    Zero) => self,
          (Plus, Plus) => match self.cmp(other) {
            Lt => from_slice(Minus, UintVec::sub(other.nums, self.nums)),
            Eq => zero(),
            Gt => from_slice(Plus,  UintVec::sub(self.nums, other.nums))
          },
          (Plus,  Minus) =>   self     + (-other),
          (Minus, Plus ) => -((-self)  + other),
          (Minus, Minus) =>   (-other) - (-self)
        }
    }

    pure fn mul(&&other: BigInt) -> BigInt {
        match (self.sign, other.sign) {
          (Zero, _    ) | (_,     Zero ) => zero(),
          (Plus, Plus ) | (Minus, Minus) => from_slice(Plus,  UintVec::mul(self.nums, other.nums)),
          (Plus, Minus) | (Minus, Plus ) => from_slice(Minus, UintVec::mul(self.nums, other.nums))
        }
    }

    pure fn div(&&other: BigInt) -> BigInt    { self.divmod(other).first()  }
    pure fn modulo(&&other: BigInt) -> BigInt { self.divmod(other).second() }

    pure fn neg() -> BigInt { BigInt { sign: self.sign.neg(), .. self } }

    pure fn to_int() -> int {
        match self.sign {
          Zero  => 0,
          Plus  => UintVec::to_int(self.nums),
          Minus => -UintVec::to_int(self.nums)
        }
    }

    static pure fn from_int(n: int) -> BigInt {
        if n > 0 { return from_slice(Plus, &[n as uint]); }
        if n < 0 { return from_slice(Minus, &[uint::max_value - (n as uint) + 1]); }
        return zero();
    }
}

impl BigInt: ExtNum {
    pure fn abs() -> BigInt {
        match self.sign {
          Zero => self,
          _    => from_slice(Plus, self.nums)
        }
    }

    pure fn divmod(&&other: BigInt) -> (BigInt, BigInt) {
        match (self.sign, other.sign) {
          (_, Zero) => fail,
          (Zero, _) => (zero(), zero()),
          (Plus, Plus) | (Minus, Minus) => {
            let (d, m) = UintVec::divmod(self.nums, other.nums);
            (from_slice(Plus, d), from_slice(Plus, m))
          },
          (Plus, Minus) | (Minus, Plus ) => {
            let (d, m) = UintVec::divmod(self.nums, other.nums);
            (from_slice(Minus, d), from_slice(Minus, m))
          }
        }
    }

    pure fn to_uint() -> uint {
        match self.sign {
          Zero  => 0,
          Plus  => UintVec::to_uint(self.nums),
          Minus => -UintVec::to_uint(self.nums)
        }
    }

    pure fn to_str_radix(radix: uint) -> ~str {
        assert 1 < radix && radix <= 16;
        match self.sign {
            Plus  => UintVec::vec_to_str_radix(self.nums, radix),
            Minus => ~"-" + UintVec::vec_to_str_radix(self.nums, radix),
            Zero  => ~"0"
        }
    }

    static pure fn zero() -> BigInt {
        BigInt { sign: Zero, nums: UintVec::vec_from_uint(0) }
    }

    static pure fn one() -> BigInt {
        BigInt { sign: Plus, nums: UintVec::vec_from_uint(1) }
    }

    static pure fn from_uint(n: uint) -> BigInt {
        if n == 0u { zero() }
        else       { from_slice(Plus, &[n]) }
    }

    static pure fn from_str_radix(buf: &str, radis: uint) -> Option<BigInt> {
        fail
    }
}

impl BigInt : BigIntGen {
    static pure fn from_slice(s: Sign, v: &[uint]) -> BigInt {
        let vec = UintVec::vec_from_slice(v);
        if vec.is_empty() || s == Zero {
            BigInt { sign: Zero, nums: vec }
        } else {
            BigInt { sign: s, nums: vec }
        }
    }
}

impl BigInt : ToStr {
    fn to_str() -> ~str { self.to_str_radix(10) }
}

impl BigInt : FromStr {
    static fn from_str(s: &str) -> Option<BigInt> { from_str_radix::<BigInt>(s, 10) }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_convert_uint() {
        fn check_conv(u: uint, b: BigInt) {
            assert from_uint::<BigInt>(u) == b;
            assert b.to_uint() == u;
        }
        check_conv(0, zero());
        check_conv(1, from_slice(Plus, @[1]));
        check_conv(uint::max_value, from_slice(Plus, @[uint::max_value]));
    }

    #[test]
    fn test_convert_int() {
        fn check_conv(i: int, b: BigInt) {
            assert num::from_int::<BigInt>(i) == b;
            assert b.to_int() == i;
        }
        check_conv(0, zero());
        check_conv(1, from_uint(1));
        check_conv(int::min_value, from_slice(Minus, @[(-(int::min_value as i64)) as uint]));
        check_conv(int::max_value, from_slice(Plus, @[int::max_value as uint]));
    }

    #[test]
    fn test_abs() {
        assert from_slice::<BigInt>(Plus,  @[1, 1, 1]).abs() == from_slice(Plus, @[1, 1, 1]);
        assert from_slice::<BigInt>(Minus, @[1, 1, 1]).abs() == from_slice(Plus, @[1, 1, 1]);
        assert zero::<BigInt>().abs() == zero();
    }

    #[test]
    fn test_shl() {
        assert from_slice::<BigInt>(Plus,  @[1, 1, 1]) << 3 == from_slice::<BigInt>(Plus, @[1 << 3, 1 << 3, 1 << 3]);
        assert from_slice::<BigInt>(Minus,  @[1, 1, 1]) << 3 == from_slice::<BigInt>(Minus, @[1 << 3, 1 << 3, 1 << 3]);
        assert zero::<BigInt>() << 3 == zero();

        assert from_slice::<BigInt>(Plus,  @[1 << (uint::bits - 2)]) << 2 ==
            from_slice::<BigInt>(Plus, @[0, 1]);
        assert from_slice::<BigInt>(Plus,  @[1 << (uint::bits - 2)]) << 3 ==
            from_slice::<BigInt>(Plus, @[0, 2]);
        assert from_slice::<BigInt>(Plus,  @[1 << (uint::bits - 2)]) << (3 + uint::bits) ==
            from_slice::<BigInt>(Plus, @[0, 0, 2]);

        assert from_slice::<BigInt>(Plus, @[0xfedc_ba98_7654_3210, 0xfedc_ba98_7654_3210]) << 4 ==
            from_slice::<BigInt>(Plus, @[0xedcb_a987_6543_2100, 0xedcb_a987_6543_210f, 0xf]);

        assert from_slice::<BigInt>(Plus, @[0x4444_3333_2222_1111, 0x8888_7777_6666_5555]) << 16 ==
            from_slice::<BigInt>(Plus, @[0x3333_2222_1111_0000, 0x7777_6666_5555_4444, 0x8888]);
    }


    #[test]
    fn test_shr() {
        assert from_slice::<BigInt>(Plus,  @[1, 1, 1]) >> 3 ==
            from_slice::<BigInt>(Plus, @[1 << (uint::bits - 3), 1 << (uint::bits - 3)]);
        assert from_slice::<BigInt>(Minus,  @[1, 1, 1]) >> 3 ==
            from_slice::<BigInt>(Minus, @[1 << (uint::bits - 3), 1 << (uint::bits - 3)]);
        assert zero::<BigInt>() << 3 == zero();

        assert from_slice::<BigInt>(Plus,  @[1 << 2]) >> 2 ==
            from_slice::<BigInt>(Plus, @[1]);
        assert from_slice::<BigInt>(Plus, @[1, 2]) >> 3 ==
            from_slice::<BigInt>(Plus,  @[1 << (uint::bits - 2)]);
        assert from_slice::<BigInt>(Plus, @[1, 1, 2]) >> (3 + uint::bits) ==
            from_slice::<BigInt>(Plus,  @[1 << (uint::bits - 2)]);

        assert from_slice::<BigInt>(Plus, @[0xedcb_a987_6543_2100, 0xedcb_a987_6543_210f, 0xf]) >> 4 ==
            from_slice::<BigInt>(Plus, @[0xfedc_ba98_7654_3210, 0xfedc_ba98_7654_3210]);

        assert from_slice::<BigInt>(Plus, @[0x3333_2222_1111_0000, 0x7777_6666_5555_4444, 0x8888]) >> 16 ==
            from_slice::<BigInt>(Plus, @[0x4444_3333_2222_1111, 0x8888_7777_6666_5555]);
    }

    #[test]
    fn test_neg() {
        assert -from_slice::<BigInt>(Plus,  @[1, 1, 1]) == from_slice(Minus, @[1, 1, 1]);
        assert -from_slice::<BigInt>(Minus, @[1, 1, 1]) == from_slice(Plus,  @[1, 1, 1]);
        assert -zero::<BigInt>() == zero();
    }

    #[test]
    fn test_cmp() {
        let nums = [ @[1, 1, 1], @[2, 1], @[2] ].map(|v| from_slice::<BigInt>(Minus, v))
            + [ zero::<BigInt>() ]
            + [ @[2], @[1, 1], @[2, 1], @[1, 1, 1] ].map(|v| from_slice::<BigInt>(Plus, v));

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

    const sum_triples: &[(&[uint], &[uint], &[uint])] = &[
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
            let a: BigInt = from_slice(Plus, aVec);
            let b: BigInt = from_slice(Plus, bVec);
            let c: BigInt = from_slice(Plus, cVec);

            assert a + b == c;
            assert b + a == c;
            assert c + (-a) == b;
            assert c + (-b) == a;
            assert a + (-c) == (-b);
            assert b + (-c) == (-a);
            assert (-a) + (-b) == (-c);
            assert a + (-a) == zero();
        }
    }

    #[test]
    fn test_sub() {
        for sum_triples.each |elm| {
            let (aVec, bVec, cVec) = elm;
            let a: BigInt = from_slice(Plus, aVec);
            let b: BigInt = from_slice(Plus, bVec);
            let c: BigInt = from_slice(Plus, cVec);

            assert c - a == b;
            assert c - b == a;
            assert (-b) - a == (-c);
            assert (-a) - b == (-c);
            assert b - (-a) == c;
            assert a - (-b) == c;
            assert (-c) - (-a) == (-b);
            assert a - a == zero();
        }
    }

    const mul_uint_triples: &[(&[uint], uint, &[uint])] = &[
        (&[],                0, &[]),
        (&[],                3, &[]),
        (&[ 2],              0, &[]),
        (&[ 1],              1, &[ 1]),
        (&[ 2],              3, &[ 6]),
        (&[ 1,  2,  3],      3, &[ 3,  6,  9]),
        (&[ 1,  1,  1],     -1, &[-1, -1, -1]),
        (&[ 1,  2,  3],     -1, &[-1, -2, -2, 2]),
        (&[ 1,  2,  3,  4], -1, &[-1, -2, -2, -2, 3]),
        (&[-1],             -1, &[ 1, -2]),
        (&[-1, -1],         -1, &[ 1, -1, -2]),
        (&[-1, -1, -1],     -1, &[ 1, -1, -1, -2]),
        (&[-1, -1, -1, -1], -1, &[ 1, -1, -1, -1, -2]),
        (&[-1/2 + 1],        2, &[ 0,  1]),
        (&[0, -1/2 + 1],     2, &[ 0,  0,  1])
    ];

    #[test]
    fn test_mul_uint() {
        for mul_uint_triples.each |elm| {
            let (aVec, bNum, cVec) = elm;
            let a: BigInt = from_slice(Plus, aVec);
            let c: BigInt = from_slice(Plus, cVec);
            let b: BigInt = from_uint(bNum);

            assert a * b == c;
            assert (-a) * b == -c;
        }
    }

    const divmod_uint_quadruples: &[(&[uint], uint, &[uint], uint)] = &[
        (&[ 1],         2, &[],               1),
        (&[ 1,  1],     2, &[-1/2+1],         1),
        (&[ 1,  1, 1],  2, &[-1/2+1, -1/2+1], 1),
        (&[ 0,  1],    -1, &[1],              1),
        (&[-1, -1],    -2, &[2, 1],           3)
    ];
    
    #[test]
    fn test_divmod_uint() {
        for mul_uint_triples.each |elm| {
            let (aVec, bNum, cVec) = elm;
            if bNum == 0 { loop }
            let a: BigInt = from_slice(Plus, aVec);
            let c: BigInt = from_slice(Plus, cVec);
            let b: BigInt = from_uint(bNum);

            assert c.divmod(b) == (a, from_uint(0));
        }

        for divmod_uint_quadruples.each |elm| {
            let (aVec, bNum, cVec, dNum) = elm;
            let a: BigInt = from_slice(Plus, aVec);
            let c: BigInt = from_slice(Plus, cVec);
            let b: BigInt = from_uint(bNum);
            let d: BigInt = from_uint(dNum);

            assert a.divmod(b) == (c, d);
        }
    }

    const mul_triples: &[(&[uint], &[uint], &[uint])] = &[
        (&[],           &[],               &[]),
        (&[],           &[ 1],             &[]),
        (&[ 1],         &[ 1],             &[1]),
        (&[ 1],         &[ 1,  1,  1],     &[1, 1,  1]),
        (&[ 1,  2],     &[ 1,  2,  3],     &[1, 4,  7,  6]),
        (&[-1, -1],     &[-1, -1, -1],     &[1, 0, -1, -2, -1]),
        (&[-1, -1, -1], &[-1, -1, -1, -1], &[1, 0,  0, -1, -2, -1, -1]),
        (&[ 0,  0,  1], &[ 1,  2,  3],     &[0, 0,  1,  2,  3]),
        (&[ 0,  0,  1], &[ 0,  0,  0,  1], &[0, 0,  0,  0,  0,  1])
    ];

    #[test]
    fn test_mul() {
        for mul_triples.each |elm| {
            let (aVec, bVec, cVec) = elm;
            let a: BigInt = from_slice(Plus, aVec);
            let b: BigInt = from_slice(Plus, bVec);
            let c: BigInt = from_slice(Plus, cVec);

            assert a * b == c;
            assert b * a == c;
            assert (-a) * b == -c;
        }
    }

    #[test]
    fn test_divmod() {
        for mul_triples.each |elm| {
            let (aVec, bVec, cVec) = elm;
            let a: BigInt = from_slice(Plus, aVec);
            let b: BigInt = from_slice(Plus, bVec);
            let c: BigInt = from_slice(Plus, cVec);

            if b != zero() {
                assert c / b == a;
            }

            if a != zero() {
                assert c / a == b;
            }
        }
    }
}
