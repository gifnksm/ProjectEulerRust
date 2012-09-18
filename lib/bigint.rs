export BigInt;

use cmp::{ Ord, Eq };

use extcmp::{ Cmp, Eq, Lt, Gt, ExtOrd };
use extnum::{ Sign, Minus, Zero, Plus, ExtNum, zero, one, from_uint, from_str_radix };
use biguint::{ BigDigit, BigUint };

trait FromBigUint {
    static pure fn from_biguint(sign: Sign, data: BigUint) -> self;
}

struct BigInt {
    priv sign: Sign,
    priv data: BigUint
}

impl BigInt : FromBigUint {
    static pure fn from_biguint(sign: Sign, data: BigUint) -> BigInt {
        BigInt { sign: sign, data: data }
    }
}

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
    pure fn eq(&&other: BigInt) -> bool { match self.cmp(other) { Eq => true, _ => false } }
    #[inline(always)]
    pure fn ne(&&other: BigInt) -> bool { !self.eq(other) }
}

impl BigInt : Ord {
    #[inline(always)]
    pure fn lt(&&other: BigInt) -> bool { match self.cmp(other) { Lt      => true, _ => false} }
    #[inline(always)]
    pure fn le(&&other: BigInt) -> bool { match self.cmp(other) { Lt | Eq => true, _ => false} }
    #[inline(always)]
    pure fn ge(&&other: BigInt) -> bool { match self.cmp(other) { Eq | Gt => true, _ => false} }
    #[inline(always)]
    pure fn gt(&&other: BigInt) -> bool { match self.cmp(other) { Gt      => true, _ => false} }
}

// impl BigInt : ExtNum {
//     pure fn abs() -> BigInt {
//         match self.sign {
//           Zero => self,
//             _  => BigInt { sign: Plus, .. self }
//         }
//     }

//     pure fn divmod(&&other: BigInt) -> (BigInt, BigInt) {
//         fail
//     }

//     pure fn to_uint() -> uint {
//         fail
//     }

//     pure fn to_str_radix(radix: uint) -> ~str {
//         fail
//     }

//     static pure fn zero() -> BigInt { from_biguint(Zero, zero()) }
//     static pure fn one()  -> BigInt { from_biguint(Plus, one()) }

//     static pure fn from_uint(n: uint) -> BigInt {
//         if n == 0u { zero() }
//         else       { from_biguint(Plus, from_uint(n)) }
//     }

//     static pure fn parse_bytes(buf: &[u8], radix: uint) -> Option<BigInt> {
//         fail
//     }

//     static pure fn from_str_radix(s: &str, radix: uint) -> Option<BigInt> { extnum::parse_bytes(str::to_bytes(s), radix) }
// }

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_cmp() {
//         let uints = [ &[2], &[1, 1], &[2, 1], &[1, 1, 1] ].map(|data| from_slice(data));
//         let nums: ~[BigInt]
//             = vec::reversed(uints).map(|bu| from_biguint(Minus, bu))
//             + [ zero() ]
//             + uints.map(|bu| from_biguint(Plus, bu));

//         for uint::range(0, nums.len()) |i| {
//             for uint::range(i, nums.len()) |j| {
//                 if i == j {
//                     assert nums[i].cmp(nums[j]) == Eq;
//                 } else {
//                     assert nums[i].cmp(nums[j]) == Lt;
//                     assert nums[j].cmp(nums[i]) == Gt;
//                 }
//             }
//         }
//     }
// }
