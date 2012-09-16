export BigInt;

use extnum::{ Sign };

#[cfg(target_arch = "x86")]
#[cfg(target_arch = "arm")]
const base_bits: uint = 16;

#[cfg(target_arch = "x86_64")]
const base_bits: uint = 32;

const base: uint = 1 << (base_bits / 2);
const hi_mask: uint = (-1 as uint) << base_bits;
const lo_mask: uint = (-1 as uint) >> base_bits;

#[inline(always)]
pure fn get_hi(n: uint) -> uint { n >> base_bits }
#[inline(always)]
pure fn get_lo(n: uint) -> uint { n & lo_mask }
#[inline(always)]
pure fn split_base(n: uint) -> (uint, uint) { (get_hi(n), get_lo(n)) }

struct BigInt {
    sign: Sign,
    nums: @[uint]
}

