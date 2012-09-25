pub trait Bounded {
    static pure fn min_value() -> self;
    static pure fn max_value() -> self;
}

impl int : Bounded {
    static pure fn min_value() -> int { int::min_value }
    static pure fn max_value() -> int { int::max_value }
}

impl i8 : Bounded {
    static pure fn min_value() -> i8 { i8::min_value }
    static pure fn max_value() -> i8 { i8::max_value }
}

impl i16 : Bounded {
    static pure fn min_value() -> i16 { i16::min_value }
    static pure fn max_value() -> i16 { i16::max_value }
}

impl i32 : Bounded {
    static pure fn min_value() -> i32 { i32::min_value }
    static pure fn max_value() -> i32 { i32::max_value }
}

impl i64 : Bounded {
    static pure fn min_value() -> i64 { i64::min_value }
    static pure fn max_value() -> i64 { i64::max_value }
}

impl uint : Bounded {
    static pure fn min_value() -> uint { uint::min_value }
    static pure fn max_value() -> uint { uint::max_value }
}

impl u8 : Bounded {
    static pure fn min_value() -> u8 { u8::min_value }
    static pure fn max_value() -> u8 { u8::max_value }
}

impl u16 : Bounded {
    static pure fn min_value() -> u16 { u16::min_value }
    static pure fn max_value() -> u16 { u16::max_value }
}

impl u32 : Bounded {
    static pure fn min_value() -> u32 { u32::min_value }
    static pure fn max_value() -> u32 { u32::max_value }
}

impl u64 : Bounded {
    static pure fn min_value() -> u64 { u64::min_value }
    static pure fn max_value() -> u64 { u64::max_value }
}