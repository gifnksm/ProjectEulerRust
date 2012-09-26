extern mod euler;

use euler::extnum::{ one, from_uint };
use euler::bigint::{ BigInt };

fn main() {
    let mut i = one::<BigInt>();
    for uint::range(0, 1000) |_n| {
        i = i * from_uint(2);
    }
    let sum = do str::byte_slice(i.to_str()) |buf| {
        buf.map(|c| *c - ('0' as u8)).foldl(0, |s, e| s + (e as uint))
    };
    io::println(fmt!("%u", sum));
}

