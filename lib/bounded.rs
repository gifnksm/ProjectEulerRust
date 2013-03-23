pub trait Bounded {
    fn min_value() -> Self;
    fn max_value() -> Self;
}

impl Bounded for int {
    fn min_value() -> int { int::min_value }
    fn max_value() -> int { int::max_value }
}

impl Bounded for i8 {
    fn min_value() -> i8 { i8::min_value }
    fn max_value() -> i8 { i8::max_value }
}

impl Bounded for i16 {
    fn min_value() -> i16 { i16::min_value }
    fn max_value() -> i16 { i16::max_value }
}

impl Bounded for i32 {
    fn min_value() -> i32 { i32::min_value }
    fn max_value() -> i32 { i32::max_value }
}

impl Bounded for i64 {
    fn min_value() -> i64 { i64::min_value }
    fn max_value() -> i64 { i64::max_value }
}

impl Bounded for uint {
    fn min_value() -> uint { uint::min_value }
    fn max_value() -> uint { uint::max_value }
}

impl Bounded for u8 {
    fn min_value() -> u8 { u8::min_value }
    fn max_value() -> u8 { u8::max_value }
}

impl Bounded for u16 {
    fn min_value() -> u16 { u16::min_value }
    fn max_value() -> u16 { u16::max_value }
}

impl Bounded for u32 {
    fn min_value() -> u32 { u32::min_value }
    fn max_value() -> u32 { u32::max_value }
}

impl Bounded for u64 {
    fn min_value() -> u64 { u64::min_value }
    fn max_value() -> u64 { u64::max_value }
}